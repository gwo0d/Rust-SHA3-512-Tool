use sha3::{Digest, Sha3_512};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let to_hash = &args[1];

    let mut hasher = Sha3_512::new();

    hasher.update(to_hash.as_bytes());

    let result = hasher.finalize();
    let hex_digest = hex::encode(result);
    
    println!("Hash: {}", hex_digest);
}