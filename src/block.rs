use super::*;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Block {
        let timestamp = now();
        let empty_string = String::new();
        let nonce = 0u64;
        Block {
            timestamp,
            pre_hash: empty_string.clone(),
            transaction: transactions,
            hash: empty_string,
            nonce,
        }
    }

    pub fn set_hash(&mut self) {
        self.hash = calculate_hash(
            &self.pre_hash,
            &self.transaction,
            &self.timestamp,
            &self.nonce,
        );
    }

    pub fn set_pre_hash(&mut self, pre_hash: String) {
        self.pre_hash = pre_hash;
    }

    pub fn mine(&mut self) {
        let target = get_difficult_string();
        while &self.hash[..DIFFICULTY_LEVEL as usize] != target {
            self.nonce += 1;
            self.hash = calculate_hash(
                &self.pre_hash,
                &self.transaction,
                &self.timestamp,
                &self.nonce,
            )
        }

        println!("Block Mined");
    }

    pub fn has_valid_transaction(&self) -> bool {
        for transaction in &self.transaction {
            if !transaction.is_valid_transaction() {
                return false
            }
        }
        true
    }
}
