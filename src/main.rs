// use crypto::digest::Digest;
// use crypto::sha3::Sha3;
use sha3::{Digest, Sha3_256};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let current_challenge = "rETH";
    let solution: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let threads = 14; // 设置线程数量
    let handles: Vec<_> = (0..threads).map(|_| {
        let solution = Arc::clone(&solution);
        thread::spawn(move || {
            loop {
                let random_value = random_bytes(32);
                let potential_solution = hexlify(&random_value);
                let hashed_solution = keccak256(&[potential_solution.as_bytes(), current_challenge.as_bytes()]);

                if hashed_solution.starts_with("0x7777777") {
                    let mut solution = solution.lock().unwrap();
                    *solution = Some(potential_solution);
                    println!("hashed_solution: {}", hashed_solution);
                    println!("solution: {}", solution.as_ref().unwrap());
                    break;
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn random_bytes(len: usize) -> Vec<u8> {
    // Generate random bytes using your preferred method
    // Replace this placeholder implementation with your own code
    // This is just an example using a fixed sequence of bytes
    (0..len).map(|i| i as u8).collect()
}

fn hexlify(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn keccak256(data: &[&[u8]]) -> String {
    let mut hasher = Sha3_256::new();
    for chunk in data {
        hasher.update(chunk);
    }
    let result = hasher.finalize();
    hexlify(&result.to_owned())
}