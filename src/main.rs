use chrono::offset::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}

impl Block {
    pub fn new(pre_hash: String, transactions: Vec<Transaction>) -> Block {

        let timestamp = Utc::now().timestamp();
        let new_hash = calculate_hash(&pre_hash, &transactions, timestamp.clone());
        Block {
            timestamp,
            pre_hash,
            transaction: transactions,
            hash: new_hash,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() ->  Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block)
    }
}

pub fn calculate_hash(
    pre_hash: &String,
    transactions: &Vec<Transaction>,
    timestamp: i64,
) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transactions
            .iter()
            .flat_map(|transaction | bincode::serialize(transaction).unwrap())
            .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());

    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}


fn main() {
    let mut blockchain = Blockchain::new();

    let genesis_block = Block::new(
        "0".to_owned(),
        vec![Transaction {
            sender: String::from("Ryan"),
            receiver: String::from("Dan"),
            amount: 200.0,
        }],
    );

    let first_block = Block::new(
        genesis_block.hash.to_owned(),
        vec![Transaction {
            sender: String::from("Sam"),
            receiver: String::from("Michael"),
            amount: 2500.0,
        }],
    );

    let second_block = Block::new(
        first_block.hash.to_owned(),
        vec![Transaction {
            sender: String::from("Michael"),
            receiver: String::from("Dan"),
            amount: 1000.0,
        }],
    );

    blockchain.add_block(genesis_block);
    blockchain.add_block(first_block);
    blockchain.add_block(second_block);

    println!("{:#?}", blockchain);
}
