use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::hash_util;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<String>,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<String>,
        previous_hash: String
    ) -> Self {
        let timestamp = Self::current_timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new()
        };

        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );

        hash_util::calculate_sha256(&data)
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}