//use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use num_traits::One;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex::encode;
use std::time::Instant;
use rayon::prelude::*;
/*
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
*/


//todo: move into inputs


const HASH_BYTE_SIZE: usize = 32;
pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug, Clone)]
pub struct Block {
    prev_block_hash: Sha256Hash,
    //prev_block_hash_string: String,
    nonce: u64,

    data: Vec<u8>,
    //data_string: String,

    hash: Sha256Hash,
    //hash_string: String,

    mined: bool,
    mining_time: std::time::Duration,
}

impl Block {
    pub fn new(data: &str, prev_hash: Sha256Hash) -> Self {
        let mut s = Self {
            prev_block_hash: prev_hash,
            nonce: 0,
            data: data.to_owned().into(),
            hash: hash_without_nonce(data, prev_hash),
            mined: false,
            mining_time: {
                let before = Instant::now();
                before.elapsed()
            },
        };
        s
    }

    pub fn genesis(data: &str) -> Self {
        Self::new(data, Sha256Hash::default())
    }

    pub fn mine_block(&mut self, max_nonce: u64, leading_zeros: u64) -> &Self{
        let before = Instant::now();
        let mut nonce_vec = create_nonce_vec(max_nonce);
        let mut rv = &mut nonce_vec;
        let rr1 = &mut rv;
        let data: &str = std::str::from_utf8(&self.data).unwrap();
        let nonced = find_nonce(data, self.prev_block_hash, rr1, leading_zeros);
        let elapsed = before.elapsed();
        if nonced != 0 {
            self.nonce = nonced;
            self.hash = nonced_hash(data, self.prev_block_hash, nonced);
            self.mined = true;
            self.mining_time = elapsed;
            self
        } else {
            self
        }

    }
}


pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(data: &str) -> Self {
        let blocks = Block::genesis(data);
        Self {
            blocks: vec![blocks]
        }
    }

    pub fn add_block(&mut self, data: &str, max_nonce: u64, leading_zeros: u64) {
        if !self.blocks.last().unwrap().mined {
            let prev_block = &mut self.blocks.pop().unwrap();
            Block::mine_block(prev_block, max_nonce, leading_zeros);
            self.blocks.push(prev_block.to_owned());
        }
        let block = Block::new(data, self.blocks.last().unwrap().hash);
        self.blocks.push(block);
    }
}

fn main() {
    let leading_zeros: u64 = 3;
    let max_nonce = 1_000_000;
    let input = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
    let before = Instant::now();
    let mut nonce_vec: Vec<u64> = create_nonce_vec(max_nonce);
    let vector_creation_time = before.elapsed();

    println!("Vector Creation Time: {:2?}", vector_creation_time);
    let mut rv = &mut nonce_vec;
    let rr1 = &mut rv;
    //let input = "";
    let before = Instant::now();
    let prev_hash = Sha256Hash::default(); //genesis block
    let nonce = find_nonce(input, prev_hash, rr1 , leading_zeros);
    let elapsed = before.elapsed();
    println!("old hash: {}", hash_without_nonce_string(input, prev_hash));
    println!("nonce: {}", nonce);
    println!("nonced hash: {}", nonced_hash_string(input, prev_hash, nonce));
    println!("Time: {:2?}", elapsed);
    let input = "trevor";
    let prev_hash = hash_without_nonce(input, prev_hash);
    let before = Instant::now();
    let nonce = find_nonce(input, prev_hash, rr1 , leading_zeros);
    println!("old hash: {}", hash_without_nonce_string(input, prev_hash));
    let elapsed = before.elapsed();
    
    println!("Nonce: {:?}", nonce);
    println!("Time: {:2?}", elapsed);
    println!("nonced hash: {}", nonced_hash_string(input, prev_hash, nonce));

    let mut bc = Blockchain::new(input);
    let blocka = &mut bc.blocks[0];
    //let mined = Block::mine_block(blocka, max_nonce, leading_zeros);
    println!("{:?}", blocka);

    Blockchain::add_block(&mut bc, "block2", max_nonce, leading_zeros);
    let blockb = &mut bc.blocks[1];
    let mined = Block::mine_block(blockb, max_nonce, leading_zeros);
    println!("{:?}", mined);
}

pub fn create_nonce_vec(max_nonce: u64) -> Vec<u64> {
    let nonce_vec: Vec<u64> = (1..(max_nonce + 1)).collect();
    //assert_eq!(max_nonce, nonce_vec[max_nonce as usize]);
    nonce_vec
}



pub fn find_nonce(input: &str, prev_hash: Sha256Hash, nonce_vec: & mut Vec<u64>, leading_zeros: u64) -> u64 {
    let nonce_wrapped = nonce_vec.par_iter().find_any(|&&nonce| found_nonce(input, prev_hash, nonce, leading_zeros));
    let nonce = *nonce_wrapped.unwrap_or(&0);
    nonce
}



pub fn nonced_hash_string(input: &str, prev_hash: Sha256Hash, nonce: u64) -> String {
    let mut contents = Vec::new();
    contents.extend_from_slice(&prev_hash);
    contents.extend_from_slice(input.as_bytes());
    contents.extend_from_slice(&convert_u64_little_endian(nonce));
    let mut hasher = Sha256::new();
    hasher.input(&contents);
    let mut hash = Sha256Hash::default();
    hasher.result(&mut hash);
    let result: String = hex::encode(hash);
    result
}

pub fn nonced_hash(input: &str, prev_hash: Sha256Hash, nonce: u64) -> Sha256Hash {
    let mut contents = Vec::new();
    contents.extend_from_slice(&prev_hash);
    contents.extend_from_slice(input.as_bytes());
    contents.extend_from_slice(&convert_u64_little_endian(nonce));
    let mut hasher = Sha256::new();
    hasher.input(&contents);
    let mut hash = Sha256Hash::default();
    hasher.result(&mut hash);
    let result = hash;
    result
}

pub fn found_nonce(input: &str, prev_hash: Sha256Hash, nonce: u64, leading_zeros: u64) -> bool {
    let max_hash = BigUint::one() << (256 - 4 * leading_zeros); //>
    let mut contents = Vec::new();
    contents.extend_from_slice(&prev_hash);
    contents.extend_from_slice(input.as_bytes());
    contents.extend_from_slice(&convert_u64_little_endian(nonce));
    let mut hasher = Sha256::new();
    hasher.input(&contents);
    let mut hash = Sha256Hash::default();
    hasher.result(&mut hash);
    let hash_int = BigUint::from_bytes_be(&hash);
    //println!("{}", hash.to_hex());
    if hash_int < max_hash {
        //println!("{}", nonce);
        true
    } else {
        false
    }
}

pub fn hash_without_nonce_string(input: &str, prev_hash: Sha256Hash) -> String {
    let mut contents = Vec::new();
    contents.extend_from_slice(&prev_hash);
    contents.extend_from_slice(input.as_bytes());
    
    let mut hasher = Sha256::new();
    hasher.input(&contents);
    let mut hash = Sha256Hash::default();
    hasher.result(&mut hash);
    let result = hex::encode(hash);
    result
}

pub fn hash_without_nonce(input: &str, prev_hash: Sha256Hash) -> Sha256Hash {
    let mut contents = Vec::new();
    contents.extend_from_slice(&prev_hash);
    contents.extend_from_slice(input.as_bytes());
    
    let mut hasher = Sha256::new();
    hasher.input(&contents);
    let mut hash = Sha256Hash::default();
    hasher.result(&mut hash);
    let result = hash;
    result
}

pub fn convert_u64_little_endian(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8) as u8,
        (val >> 16) as u8,
        (val >> 24) as u8,
        (val >> 32) as u8,
        (val >> 40) as u8,
        (val >> 48) as u8,
        (val >> 56) as u8,
    ]
}