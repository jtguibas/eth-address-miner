use std::env;
use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use std::thread;
use std::vec::Vec;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time;

fn manager(iterations: Arc<AtomicUsize>, found: Arc<AtomicBool>) {
    let mut counter = 0;
    while !found.load(Ordering::SeqCst) {
        let past_iterations = iterations.load(Ordering::SeqCst);
        thread::sleep(time::Duration::from_secs(1));
        let curr_iterations = iterations.load(Ordering::SeqCst);
        print!("[{}] iterations / sec = {}, total iterations = {}\n",
               counter, curr_iterations - past_iterations, curr_iterations);
        counter += 1;
    }
}

fn find(prefix: String, iterations: Arc<AtomicUsize>, found: Arc<AtomicBool>) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let mut hasher = Sha3::keccak256();

    while !found.load(Ordering::SeqCst) {
        let (private_key, public_key) = secp.generate_keypair(&mut rng);
        let serialized_public_key = &public_key.serialize_uncompressed()[1..];
        hasher.input(&serialized_public_key);
        let address = &hasher.result_str()[24..];
        hasher.reset();

        if prefix.eq(&address[..prefix.chars().count()]) {
            found.store(true, Ordering::SeqCst);
            thread::sleep(time::Duration::from_secs(1));
            println!("\n\x1b[32mmatch found!\x1b[0m");
            print!("private key: 0x{}\n", private_key.display_secret());
            print!("public key: 0x{}\n", public_key.to_string());
            print!("address: 0x{}\n", address);
        }

        iterations.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    print!("eth-address-miner\n");
    let args: Vec<String> = env::args().collect();
    let prefix = &args[1];
    let num_cpus = num_cpus::get();

    print!("> searching for prefix: {}\n", prefix);
    print!("> number of cpus: {}\n\n", num_cpus);

    let mut threads : Vec<thread::JoinHandle<()>> = Vec::new();
    let iterations = Arc::new(AtomicUsize::new(0));
    let found = Arc::new(AtomicBool::new(false));

    for i in 0..(num_cpus + 1) {
        let my_prefix = prefix.clone();
        let my_iterations = iterations.clone();
        let my_found = found.clone();
        if i < num_cpus {
            threads.push(thread::spawn(move || {
                find(my_prefix, my_iterations, my_found);
            }));
        } else {
            threads.push(thread::spawn(move || {
                manager(my_iterations, my_found);
            }))
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}