use std::collections::HashMap;
use sha2::{Sha256, Digest};
use std::io::{self, Read};
use std::fs::File;
use chrono::Local;
use colored::*;

pub struct Node {
    hash: String, 
    next: Option<Box<Node>>,
    timestamp: String
}

impl Node {

    pub fn new(file_path: &str) -> Self {
        
        let hashed = Self::hash_file(file_path); 
        let ts = Local::now().to_string();   

        let node = Node {
            hash: hashed.expect("REASON"),
            next: None, 
            timestamp: ts
        };

        node
    }

    fn hash_file(file_path: &str) -> io::Result<String> {
        let mut f = File::open(file_path)?; 
        let mut sha = Sha256::new(); 
        let mut buf = [0u8; 4096]; 

        loop {
            let bytes = f.read(&mut buf)?;
            if bytes == 0 {break;}
            sha.update(&buf[..bytes])
        }

        Ok(format!("{:x}", sha.finalize()))
    }

    pub fn print_node(&self) {
        let hash = self.hash.clone(); 
        let ts = self.timestamp.clone(); 
        println!(
            "{}: {}\n{}: {}\n", 
            "Hash".green(), hash, 
            "Timestamp".yellow(), ts
        );
    }

}
