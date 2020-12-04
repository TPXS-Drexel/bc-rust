//use sha2::{Sha256, Digest};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex::encode;
use num_bigint::BigUint;
use num_traits::One;
use rayon::prelude::*;
use std::time::Instant;
/*
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
*/
//serde
// use async_std::fs;
// use futures::future;
// use std::path::Path;

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

use actix_web::{error, post, web, App, Error, HttpResponse};
use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//todo: move into inputs

const HASH_BYTE_SIZE: usize = 32;
pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Serialize, Deserialize)]
pub struct Input {
    content: String,
    leading_zeros: u64,
    max_nonce: u64,
}

// impl Input {
//     pub fn new(content: &str, leading_zeros: u64, max_nonce: u64) -> Self {
//         let mut s = Self {
//             content: content.to_owned().into(),
//             leading_zeros: leading_zeros.to_owned().into(),
//             max_nonce: max_nonce.to_owned().into(),
//         };
//         s
//     }

//     pub fn genesis(content: &str, leading_zeros: u64, max_nonce: u64) -> Self {
//         Self::new(content, leading_zeros, max_nonce)
//     }
// }

// pub struct QueueInput {
//     inputs: Vec<Input>,
// }

// impl QueueInput {
//     pub fn new(content: &str, leading_zeros: u64, max_nonce: u64) -> Self {
//         let inputs = Input::genesis(content, leading_zeros, max_nonce);
//         Self {
//             inputs: vec![inputs],
//         }
//     }

//     pub fn add_input(&mut self, content: &str, max_nonce: u64, leading_zeros: u64) {
//         let input = Input::new(content, leading_zeros, max_nonce);
//         self.inputs.push(input);
//     }
// }

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
}

impl Block {
    pub fn new(mut id: u64, data: &str, prev_hash: Sha256Hash) -> Self {
        let mut s = Self {
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
        };
        s
    }

    pub fn genesis(mut id: u64, data: &str) -> Self {
        Self::new(id, data, Sha256Hash::default())
    }

    pub fn mine_block(&mut self, max_nonce: u64, leading_zeros: u64) -> &Self {
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

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(id: u64, data: &str) -> Self {
        let blocks = Block::genesis(id, data);
        Self {
            blocks: vec![blocks],
        }
    }

    pub fn add_block(&mut self, id: u64, data: &str, max_nonce: u64, leading_zeros: u64) {
        if !self.blocks.last().unwrap().mined {
            let prev_block = &mut self.blocks.pop().unwrap();
            Block::mine_block(prev_block, max_nonce, leading_zeros);
            self.blocks.push(prev_block.to_owned());
        }
        let block = Block::new(id, data, self.blocks.last().unwrap().hash);
        self.blocks.push(block);
    }
}

// async fn get_input<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
//     let path = path.as_ref().to_owned();
//     future::ok(path.to_)
// }

fn get_request_content(text: &str) -> &str {
    println!("doing regex");
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(GET /convert_bc(.*) HTTP)").unwrap();
    }
    match RE.captures(text) {
        Some(caps) => {
            println!("Found {}", &caps[0]);
            let end_pos = &caps[0].len() - 5;
            // let slice = &caps[0][18..end_pos];
            // println!("slice is {} ", slice);
            return &caps.get(0).unwrap().as_str()[18..end_pos];
        }
        None => {
            println!("Can't find {}", text);
            return text;
        }
    }
}

fn main() {
    // let mut queue_input = QueueInput::new("a", 3, 1_000_000);
    // QueueInput::add_input(&mut queue_input, "b", 3, 1_000_000);
    // QueueInput::add_input(&mut queue_input, "c", 3, 1_000_000);
    // QueueInput::add_input(&mut queue_input, "Blakeau", 3, 1_000_000);
    // let input1 = Input::new("a", 3, 1_000_000);
    // let input2 = Input::new("b", 3, 1_000_000);
    // let input3 = Input::new("c", 3, 1_000_000);
    // println!("______________ {:?}", queue_input.inputs[3].content);

    // //INPUTS - THESE INFO WILL BE PARSED FROM FRONTEND
    // let leading_zeros: u64 = 3;
    // let max_nonce = 1_000_000;
    // let input = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";
    // //END INPUTS
    // let before = Instant::now();
    // let mut nonce_vec: Vec<u64> = create_nonce_vec(max_nonce);
    // let vector_creation_time = before.elapsed();

    // println!("Vector Creation Time: {:2?}", vector_creation_time);
    // let mut rv = &mut nonce_vec;
    // let rr1 = &mut rv;
    // //let input = "";
    // let before = Instant::now();
    // let prev_hash = Sha256Hash::default(); //genesis block
    // let nonce = find_nonce(input, prev_hash, rr1, leading_zeros);
    // let elapsed = before.elapsed();
    // println!("old hash: {}", hash_without_nonce_string(input, prev_hash));
    // println!("nonce: {}", nonce);
    // println!(
    //     "nonced hash: {}",
    //     nonced_hash_string(input, prev_hash, nonce)
    // );
    // println!("Time: {:2?}", elapsed);
    // let input = "Trevor";
    // let leading_zeros: u64 = 3;
    // let max_nonce = 1_000_000;
    // let prev_hash = hash_without_nonce(input, prev_hash);
    // let before = Instant::now();
    // let nonce = find_nonce(input, prev_hash, rr1, leading_zeros);
    // println!("old hash: {}", hash_without_nonce_string(input, prev_hash));
    // let elapsed = before.elapsed();
    // println!("Nonce: {:?}", nonce);
    // println!("Time: {:2?}", elapsed);
    // println!(
    //     "nonced hash: {}",
    //     nonced_hash_string(input, prev_hash, nonce)
    // );

    //demo of functions above ^^

    // let mut file = File::create("output.json").expect("Could not create file");

    // Initiate values when function first start
    let init_id = 0;
    let init_input = "Null";
    let init_leading_zeros: u64 = 3;
    let init_max_nonce = 1_000_000;
    let mut n: u64 = 0;

    //ACTUAL USE OF FUNCTIONS:
    //TAKE FIRST INPUT
    //MATCH STATEMENT FOR LEADING ZEROS CHANGED
    let mut bc = Blockchain::new(init_id, init_input);

    //todo: make threads for server i/o

    //BEGIN A LOOP
    let blocka = &mut bc.blocks[0];
    //SOME MATCH STATEMENT
    let mined = Block::mine_block(blocka, init_max_nonce, init_leading_zeros);
    n += 1;

    println!("{:?}", blocka);

    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        // Handle connection from stream;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        // Read and process HTTP request
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("Request: {}", message);
        // Declare 2 expected cases of HTTP request
        let get = b"GET / HTTP/1.1\r\n";
        let get_convert = b"GET /convert_bc";
        // Triage flow based on HTTP request content
        let (status_line, filename) = 
        // If HTTP 200 OK
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "./user_interface/index.html")
        // If HTTP sends content for new block
        } else if buffer.starts_with(get_convert) {
            let mut file = File::create("output.json").expect("Could not create file");
            println!("Blockchain now");
            let input_content = get_request_content(&message);
            println!("{}", input_content);
            // let result = serde_json::from_str(input_content);
            // if result.is_ok() {
            // let p = input_content;
            println!("Content is {}", input_content);
            Blockchain::add_block(
                &mut bc,
                n,
                input_content,
                init_max_nonce,
                init_leading_zeros,
            );
            let new_block = &mut bc.blocks[n as usize];
            let mined = Block::mine_block(new_block, init_max_nonce, init_leading_zeros);
            n += 1;
            println!("{:?}", mined);
            println!("{:?}", hex::encode(mined.hash));
            let j = serde_json::to_string(&bc.blocks).unwrap();
            println!("{}", j);
            file.write_all(j.as_ref()).expect("Cannot write the file");
            // }
            ("HTTP/1.1 200 OK\r\n\r\n", "./user_interface/index.html")
        // If HTTP sends an unrecognized request
        } else {
            (
                "HTTP/1.1 404 NOT FOUND\r\n\r\n",
                "./user_interface/index_2.html",
            )
        };
        // Compile and return content
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    // Receive initial data from front-end
    // let input_json_str = r#"
    //     {
    //         "content": "",
    //         "leading_zeros": 3,
    //         "max_nonce": 1000000
    //     }"#;

    // Parse string into serde_json::Value
    // hash_input(input_json_str, init_leading_zeros, init_max_nonce, mut blockchain: Blockchain, n);

    // for i in 0..4 {
    //     println!("----------{:?}", queue_input.inputs[i].content);
    //     Blockchain::add_block(
    //         &mut bc,
    //         queue_input.inputs[i].content,
    //         init_max_nonce,
    //         init_leading_zeros,
    //     );
    //     let blockb = &mut bc.blocks[1];
    //     let mined = Block::mine_block(blockb, init_max_nonce, init_leading_zeros);
    //     println!("{:?}", mined);
    //     println!("{:?}", hex::encode(mined.hash));
    //     let j = serde_json::to_string(&bc);
    //     println!("{:?}", j);
    // }

    //to change to hex
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
