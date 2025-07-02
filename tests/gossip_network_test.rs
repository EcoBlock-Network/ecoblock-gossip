use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_core::{TangleBlockData, SensorData};
use ecoblock_gossip::node::gossip_node::GossipNode;
use ecoblock_storage::tangle::block::TangleBlock;

#[test]
fn test_block_propagation_line_network() {
    let keypair = CryptoKeypair::generate();

    let data = TangleBlockData {
        parents: vec![],
        data: SensorData {
            pm25: 1.0,
            co2: 400.0,
            temperature: 22.0,
            humidity: 50.0,
            timestamp: 42,
        },
    };

    let block = TangleBlock::new(data, &keypair);

    let node_a = Arc::new(Mutex::new(GossipNode::new("A")));
    let node_b = Arc::new(Mutex::new(GossipNode::new("B")));
    let node_c = Arc::new(Mutex::new(GossipNode::new("C")));

    node_a.lock().unwrap().add_peer(Arc::clone(&node_b));
    node_b.lock().unwrap().add_peer(Arc::clone(&node_c));

    let mut visited = HashSet::new();
    node_a.lock().unwrap().receive_block(block.clone(), &mut visited);

    assert!(node_a.lock().unwrap().has_block(&block.id));
    assert!(node_b.lock().unwrap().has_block(&block.id));
    assert!(node_c.lock().unwrap().has_block(&block.id));
}
