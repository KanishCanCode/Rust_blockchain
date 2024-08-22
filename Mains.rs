use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// Block structure
#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

// Blockchain structure
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<String>,
}

impl Blockchain {
    /// Creates a new Blockchain with a genesis block.
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

    /// Adds a block to the blockchain with the given data.
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Blockchain should have at least one block.");
        let timestamp = current_timestamp();
        let hash = self.calculate_hash(&previous_block.hash, timestamp, &data);

        let block = Block {
            index: self.chain.len() as u32,
            timestamp,
            data,
            previous_hash: previous_block.hash.clone(),
            hash,
        };
        self.chain.push(block);
    }

    /// Calculates the hash of the block using previous hash, timestamp, and data.
    fn calculate_hash(&self, previous_hash: &str, timestamp: u64, data: &str) -> String {
        let mut hasher = DefaultHasher::new();
        previous_hash.hash(&mut hasher);
        timestamp.hash(&mut hasher);
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Adds a transaction to the pending transactions list.
    fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    /// Mines a new block with all pending transactions.
    fn mine_block(&mut self) {
        if self.pending_transactions.is_empty() {
            return;
        }
        let data = self.pending_transactions.join(";");
        self.add_block(data);
        self.pending_transactions.clear();
    }
}

/// Returns the current timestamp in seconds since UNIX_EPOCH.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_transaction("Transaction 1".to_string());
    blockchain.add_transaction("Transaction 2".to_string());
    blockchain.mine_block();

    blockchain.add_transaction("Transaction 3".to_string());
    blockchain.mine_block();

    println!("Blockchain:");
    for block in &blockchain.chain {
        println!(
            "Block {} - Hash: {} - Data: {}",
            block.index, block.hash, block.data
        );
    }
}
