use super::block::Block;

pub struct BlockChain {
    pub chain: Vec<Block>,
    pub difficulty: usize
}

impl BlockChain {

    pub fn new() -> Self {
        let mut block_chain = BlockChain {
            chain: vec![],
            difficulty: 4,
        };
        block_chain.create_genesis_block();
        block_chain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], String::new());
        self.chain.push(genesis_block);
    }

    fn add_block(&mut self, transactions: Vec<String>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        block.mine_block(self.difficulty);
        self.chain.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}