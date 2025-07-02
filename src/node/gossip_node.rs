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
        if self.engine.has_received(&block.id) {
            println!("🔁 NODE {}: Already has block {}", self.id, block.id);
            return;
        }

        self.engine.propagate_block(&block);
        visited.insert(self.id.clone());

        let peers = self.peers.clone(); // clone avant lock

        for peer in peers {
            let peer_id = peer.lock().unwrap().id.clone();
            if visited.contains(&peer_id) {
                continue;
            }

            let block_clone = block.clone();
            peer.lock().unwrap().receive_block(block_clone, visited);
        }
    }

    pub fn has_block(&self, block_id: &str) -> bool {
        self.engine.has_received(block_id)
    }
}
