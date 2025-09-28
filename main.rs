use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// Block structure
#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

// Blockchain structure
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<String>,
    difficulty: usize, // number of leading zeros required in PoW
}

impl Blockchain {
    /// Creates a new Blockchain with a genesis block.
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: current_timestamp(),
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "0".to_string(),
            nonce: 0,
        };
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
            difficulty,
        }
    }

    /// Adds a transaction to the pending transactions list.
    fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    /// Mines a new block with all pending transactions.
    fn mine_block(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No transactions to mine.");
            return;
        }

        let data = self.pending_transactions.join(";");
        let previous_block = self.chain.last().unwrap();
        let index = self.chain.len() as u32;
        let timestamp = current_timestamp();

        println!("⛏️ Mining block {} ...", index);

        let (nonce, hash) = self.proof_of_work(&previous_block.hash, timestamp, &data);

        let block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_block.hash.clone(),
            hash,
            nonce,
        };

        self.chain.push(block);
        self.pending_transactions.clear();
    }

    /// Proof-of-Work: find nonce that produces a hash with required leading zeros.
    fn proof_of_work(&self, previous_hash: &str, timestamp: u64, data: &str) -> (u64, String) {
        let mut nonce = 0;
        loop {
            let hash = self.calculate_hash(previous_hash, timestamp, data, nonce);
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    /// Calculates the hash of the block.
    fn calculate_hash(&self, previous_hash: &str, timestamp: u64, data: &str, nonce: u64) -> String {
        let mut hasher = DefaultHasher::new();
        previous_hash.hash(&mut hasher);
        timestamp.hash(&mut hasher);
        data.hash(&mut hasher);
        nonce.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Validates the entire blockchain.
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated_hash = self.calculate_hash(
                &current.previous_hash,
                current.timestamp,
                &current.data,
                current.nonce,
            );

            if current.hash != recalculated_hash {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
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
    let mut blockchain = Blockchain::new(3); // Difficulty = 3 (3 leading zeros)

    blockchain.add_transaction("Alice pays Bob 5 BTC".to_string());
    blockchain.add_transaction("Bob pays Charlie 2 BTC".to_string());
    blockchain.mine_block();

    blockchain.add_transaction("Charlie pays Dave 1 BTC".to_string());
    blockchain.mine_block();

    println!("\n📜 Blockchain:");
    for block in &blockchain.chain {
        println!(
            "Block {} | Nonce: {} | Hash: {} | Data: {}",
            block.index, block.nonce, block.hash, block.data
        );
    }

    println!("\n✅ Blockchain valid? {}", blockchain.is_valid());
}
