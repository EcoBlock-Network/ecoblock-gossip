use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use ecoblock_storage::tangle::block::TangleBlock;
use crate::engine::gossip::GossipEngine;

#[derive(Debug)]
pub struct GossipNode {
    pub id: String,
    pub engine: GossipEngine,
    pub peers: Vec<Arc<Mutex<GossipNode>>>,
}

impl GossipNode {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            engine: GossipEngine::new(),
            peers: vec![],
        }
    }

    pub fn add_peer(&mut self, peer: Arc<Mutex<GossipNode>>) {
        self.peers.push(peer);
    }

    pub fn receive_block(&mut self, block: TangleBlock, visited: &mut HashSet<String>) {
        println!("   NODE {}: Starting receive_block", self.id);
        
        // Prevent infinite loops by checking if this node was already visited
        if visited.contains(&self.id) {
            println!("   NODE {}: Already visited, stopping propagation", self.id);
            return;
        }
        
        // Check if we already have this block
        if self.engine.has_received(&block.id) {
            println!("   NODE {}: Already has block", self.id);
            return;
        }

        // Process the block and mark node as visited
        self.engine.propagate_block(&block);
        visited.insert(self.id.clone());

        println!("   NODE {}: Received block", self.id);

        // Clone peers to avoid borrow checker issues
        let peers = self.peers.clone();
        println!("   NODE {}: Has {} peers", self.id, peers.len());

        // Propagate to all peers that haven't been visited yet
        for (i, peer) in peers.iter().enumerate() {
            // Get peer ID first, then check if it's already visited
            let peer_id = {
                let peer_lock = peer.lock().unwrap();
                peer_lock.id.clone()
            }; // Release lock immediately
            
            println!("   NODE {}: Checking peer {} (#{}/{})", self.id, peer_id, i+1, peers.len());
            
            if !visited.contains(&peer_id) {
                let block_clone = block.clone();
                println!("   NODE {}: Propagating to peer {}", self.id, peer_id);
                
                // Take lock only when needed and release immediately
                {
                    let mut peer_lock = peer.lock().unwrap();
                    peer_lock.receive_block(block_clone, visited);
                }
                
                println!("   NODE {}: Propagation to peer {} complete", self.id, peer_id);
            } else {
                println!("   NODE {}: Skipping peer {} (already visited)", self.id, peer_id);
            }
        }
        
        println!("   NODE {}: Propagation complete", self.id);
    }

    pub fn has_block(&self, block_id: &str) -> bool {
        self.engine.has_received(block_id)
    }
}
