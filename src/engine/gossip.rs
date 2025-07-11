use std::collections::HashSet;
use ecoblock_storage::tangle::block::TangleBlock;

#[derive(Debug)]
pub struct GossipEngine {
    received_block_ids: HashSet<String>,
}

impl GossipEngine {
    pub fn new() -> Self {
        Self {
            received_block_ids: HashSet::new(),
        }
    }

    pub fn propagate_block(&mut self, block: &TangleBlock) {
        // Check if we've already received this block
        if !self.received_block_ids.insert(block.id.clone()) {
            // Block already received, no need to propagate again
            return;
        }

        // New block received, propagate it
        println!("   Propagate: {}", block.id);
    }

    pub fn has_received(&self, block_id: &str) -> bool {
        self.received_block_ids.contains(block_id)
    }

    pub fn total_blocks(&self) -> usize {
        self.received_block_ids.len()
    }
}
