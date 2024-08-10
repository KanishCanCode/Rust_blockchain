
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

// Block structure
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

// Blockchain structure
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<String>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
        };
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash = self.calculate_hash(
            previous_block.hash.clone(),
            timestamp.to_string(),
            data.clone(),
        );
        let block = Block {
            index: self.chain.len() as u32,
            timestamp,
            data,
            previous_hash: previous_block.hash.clone(),
            hash,
        };
        self.chain.push(block);
    }

    fn calculate_hash(&self, previous_hash: String, timestamp: String, data: String) -> String {
        let mut hash = format!("{}{}{}", previous_hash, timestamp, data);
        // Simple hash function (not secure for production)
        hash = format!("{:x}", std::hash::Hash::hash(&hash));
        hash
    }

    fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    fn mine_block(&mut self) {
        if self.pending_transactions.is_empty() {
            return;
        }
        let data = self.pending_transactions.join("");
        self.add_block(data);
        self.pending_transactions.clear();
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_transaction("Transaction 1".to_string());
    blockchain.add_transaction("Transaction 2".to_string());
    blockchain.mine_block();
    println!("Blockchain:");
    for block in blockchain.chain {
        println!(
            "Block {} - Hash: {} - Data: {}",
            block.index, block.hash, block.data
        );
    }
}
