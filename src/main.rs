// use crypto::digest::Digest;
// use crypto::sha3::Sha3;
use sha3::{Digest, Sha3_256};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;


fn main() {
    let current_challenge = "rETH";
    let solution: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let threads = 8; // 设置线程数量
    let handles: Vec<_> = (0..threads).map(|_| {
        let solution = Arc::clone(&solution);
        thread::spawn(move || {
            loop {
                let mut rng = rand::thread_rng();
                let random_value: [u8; 32] = rng.gen();
                // let mut random_value = random_bytes(32);
                // println!("random_value: {:?}", random_value);
                let potential_solution = hexlify(&random_value);
                // const potential_solution = ethers.utils.hexlify(random_value);

                let hashed_solution = "0x".to_owned() + &keccak256(&[potential_solution.as_bytes(), current_challenge.as_bytes()]);
                // println!("hashed_solution_to_try: {}",hashed_solution);
                if hashed_solution.starts_with("0x7777777") {
                    // 0x7777777
                    let mut solution = solution.lock().unwrap();
                    *solution = Some(potential_solution);
                    println!("hashed_solution OK!: {}", hashed_solution);
                    println!("solution: {}", "0x".to_owned() + solution.as_ref().unwrap());
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