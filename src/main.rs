use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex::encode;
use num_bigint::BigUint;
use num_traits::One;
use rayon::prelude::*;
use std::time::Instant;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

// HTTP stuff
extern crate regex;
#[macro_use]
extern crate lazy_static;

//use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use std::fs;
//use std::io::prelude::*;
use std::net::TcpListener;
//use std::net::TcpStream;

//todo: move into inputs

const HASH_BYTE_SIZE: usize = 32;
pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Serialize, Deserialize)]
pub struct Input {
    content: String,
    leading_zeros: u64,
    max_nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    id: u64,
    prev_block_hash: Sha256Hash,
    //prev_block_hash_string: String,
    nonce: u64,
    input_data: String,
    data: Vec<u8>,
    //data_string: String,
    hash: Sha256Hash,
    //hash_string: String,
    mined: bool,
    mining_time: std::time::Duration,
    encoded_hash: String,
    encoded_prev_block_hash: String,
}

impl Block {
    fn new(id: u64, data: &str, prev_hash: Sha256Hash) -> Self {
        Self {
            id: id,
            prev_block_hash: prev_hash,
            nonce: 0,
            input_data: data.to_string(),
            data: data.to_owned().into(),
            hash: hash_without_nonce(data, prev_hash),
            mined: false,
            mining_time: {
                let before = Instant::now();
                before.elapsed()
            },
            encoded_hash: hex::encode(hash_without_nonce(data, prev_hash)),
            encoded_prev_block_hash: hex::encode(prev_hash),
        }
    }

    fn genesis(data: &str) -> Self {
        Self::new(0, data, Sha256Hash::default())
    }

    fn mine_block(&mut self, max_nonce: u64, leading_zeros: u64) -> &Self {
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
            self.encoded_hash = hex::encode(nonced_hash(data, self.prev_block_hash, nonced));
            self.encoded_prev_block_hash = hex::encode(self.prev_block_hash);
            self
        } else {
            self
        }
    }
    /*
    fn refresh_block(&mut self) {
        self.hash = hash_without_nonce(std::str::from_utf8(&self.data).unwrap(), self.prev_block_hash);
        self.mined = false;
        self.nonce = 0;
        self.encoded_hash = hex::encode(hash_without_nonce(std::str::from_utf8(&self.data).unwrap(), self.prev_block_hash));
    }
    */
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new(data: &str) -> Self {
        let blocks = Block::genesis(data);
        Self {
            blocks: vec![blocks],
        }
    }

    fn add_block(&mut self, id: u64, data: &str, max_nonce: u64, leading_zeros: u64) {
        if !self.blocks.last().unwrap().mined {
            let prev_block = &mut self.blocks.pop().unwrap();
            Block::mine_block(prev_block, max_nonce, leading_zeros);
            self.blocks.push(prev_block.to_owned());
        }
        let block = Block::new(id, data, self.blocks.last().unwrap().hash);
        self.blocks.push(block);
    }

    fn check_and_mine_blocks(&mut self, max_nonce: u64, leading_zeros: u64) {
        let mut blocks_rev: Vec<Block> = Vec::new();
        for block in &self.blocks {
            blocks_rev.push(block.clone());
        }
        let mut mined = true;
        let mut temp_prev_hash: Sha256Hash = Sha256Hash::default();
        for mut block in blocks_rev {
            if block.mined && mined {
                // println!("Block isn't mined {:?}", block);
                temp_prev_hash = block.hash;
            } else {
                // println!("Block before mined {:?}", block);
                mined = false;
                block.mined = mined;
                block.prev_block_hash = temp_prev_hash;
                let temp_block = Block::mine_block(&mut block, max_nonce, leading_zeros);
                // println!("Block after mined {:?}", temp_block);
                self.blocks[temp_block.id as usize] = temp_block.clone();
                temp_prev_hash = temp_block.hash;
            }
        }
    }
}

mod f_e;

fn main() {
    fs::remove_file("output.json").expect("failed to remove file");
    let mut file = File::create("output.json").expect("Could not create file");
    // default values
    let init_input = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
    let mut init_leading_zeros: u64 = 3;
    let mut init_max_nonce: u64 = 1_000_000;
    let mut n: u64 = 0;
    // Initialize the blockchain
    let mut bc = Blockchain::new(init_input);
    n += 1;

    let j = serde_json::to_string(&bc.blocks).unwrap();
    // println!("{}", j);
    file.write_all(j.as_ref()).expect("Cannot write the file");

    // Listen for incoming TCP connections on localhost port 8080
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    //expected cases of HTTP request
    let get = b"GET / HTTP/1.1\r\n";
    let get_convert = b"GET /convert_bc";
    let get_mine = b"GET /minebc";
    let get_update_pref = b"GET /setting_bc";

    // Block forever, keep on handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // Handle connection from stream;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        // Read and process HTTP request
        let message = String::from_utf8_lossy(&buffer[..]); // HTTP REQUEST sent back from front end

        // Triage flow based on HTTP request content
        let (status_line, filename) = if buffer.starts_with(get) {
            //HTTP 200 OK
            ("HTTP/1.1 200 OK\r\n\r\n", "./user_interface/index.html")
        }
        // If HTTP sends content for new block
        else if buffer.starts_with(get_convert) {
            let mut file = File::create("output.json").expect("Could not create file");
            println!("Add new block...");
            let input_content = &f_e::get_request_content(&message).replace("%20", " ");
            println!("Block content is {}", input_content);
            Blockchain::add_block(
                &mut bc,
                n,
                input_content,
                init_max_nonce,
                init_leading_zeros,
            );

            Block::mine_block(
                &mut bc.blocks[n as usize],
                init_max_nonce,
                init_leading_zeros,
            );
            let j = serde_json::to_string(&bc.blocks).unwrap();
            file.write_all(j.as_ref()).expect("Cannot write the file");
            n += 1;
            ("HTTP/1.1 200 OK\r\n\r\n", "./output.json")
        }
        // If HTTP asks to mine from a certain block
        else if buffer.starts_with(get_mine) {
            let mut file = File::create("output.json").expect("Could not create file");
            println!("Mine...");
            let mine_id = f_e::get_mine_id(&message);
            let mine_content = &f_e::get_mine_content(&message).replace("%20", " ");
            println!("Mine id is {}", mine_id);
            println!("Mine content is {}", mine_content);
            // Change content of block to mine
            bc.blocks[mine_id as usize].input_data = mine_content.to_string();
            bc.blocks[mine_id as usize].data = mine_content.to_owned().into();
            bc.blocks[mine_id as usize].mined = false;
            Blockchain::check_and_mine_blocks(&mut bc, init_max_nonce, init_leading_zeros);

            let j = serde_json::to_string(&bc.blocks).unwrap();

            file.write_all(j.as_ref()).expect("Cannot write the file");
            ("HTTP/1.1 200 OK\r\n\r\n", "./output.json")

        // If HTTP asks to update leading zeros and/or max nonce
        } else if buffer.starts_with(get_update_pref) {
            let mut file = File::create("output.json").expect("Could not create file");
            println!("Update preference...");
            init_max_nonce = f_e::get_max_nonce(&message);
            let leading_zeros = f_e::get_leading_zeros(&message);
            println!("Max nonce id is {}", init_max_nonce);
            println!("Leading zeros is {}", leading_zeros);
            if leading_zeros != init_leading_zeros {
                init_leading_zeros = leading_zeros;
                n = 0;
                bc = Blockchain::new(init_input);
                n += 1;
                let j = serde_json::to_string(&bc.blocks).unwrap();
                // println!("{}", j);
                file.write_all(j.as_ref()).expect("Cannot write the file");
                ("HTTP/1.1 200 OK\r\n\r\n", "./user_interface/index.html")
            } else {
                ("HTTP/1.1 200 OK\r\n\r\n", "./output.json")
            }

        // If HTTP sends an unrecognized request
        } else {
            ("HTTP/1.1 200 OK\r\n\r\n", "./output.json")
        };
        // Compile and return content
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

pub fn create_nonce_vec(max_nonce: u64) -> Vec<u64> {
    let nonce_vec: Vec<u64> = (1..(max_nonce + 1)).collect();
    //assert_eq!(max_nonce, nonce_vec[max_nonce as usize]);
    nonce_vec
}

pub fn find_nonce(
    input: &str,
    prev_hash: Sha256Hash,
    nonce_vec: &mut Vec<u64>,
    leading_zeros: u64,
) -> u64 {
    let nonce_wrapped = nonce_vec
        .par_iter()
        .find_any(|&&nonce| found_nonce(input, prev_hash, nonce, leading_zeros));
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
    ];
}
