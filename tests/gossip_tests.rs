use ecoblock_core::{SensorData, TangleBlockData};
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_gossip::engine::gossip::GossipEngine;
use ecoblock_storage::tangle::block::TangleBlock;

#[test]
fn test_propagate_same_block_twice_should_be_ignored() {
    let mut gossip = GossipEngine::new();
    let keypair = CryptoKeypair::generate();

    let block_data = TangleBlockData {
        parents: vec![],
        data: SensorData {
            pm25: 10.0,
            co2: 420.0,
            temperature: 22.5,
            humidity: 45.0,
            timestamp: 123456,
        },
    };

    let block = TangleBlock::new(block_data, &keypair);

    gossip.propagate_block(&block);
    assert_eq!(gossip.total_blocks(), 1);
    assert!(gossip.has_received(&block.id));

    gossip.propagate_block(&block);
    assert_eq!(gossip.total_blocks(), 1, "Le bloc ne doit pas être compté deux fois");
}
