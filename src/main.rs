mod blockchain;
mod utils;

use blockchain::blockchain::BlockChain;

fn main() {
    let mut blockchain = BlockChain::new();

    println!("Mining block 1....");
    blockchain.add_block(
        vec!["Emma to David 0.01 BTC - first transaction".to_string()]
    );

    println!("Mining block 2....");
    blockchain.add_block(
        vec!["David to Tuksa 0.2 BTC - second transaction".to_string()]
    );

    println!("Mining block 3....");
    blockchain.add_block(
        vec!["Tuksa to Emma 3 BTC - third transaction".to_string()]
    );

    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
    
    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
