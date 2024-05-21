use clap::Parser;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rug::ops::Pow;
use rug::Integer;
use std::collections::HashSet;

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(short, long, default_value_t = 32)]
    num_keys: u32,
    #[arg(short, long, default_value_t = 128)]
    key_size: u32,
}

fn main() {
    let args = Args::parse();

    gen_map(args.num_keys, args.key_size)
}

fn gen_map(num_keys: u32, key_size: u32) {
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    let num_bits = key_size / num_keys;

    if num_keys > key_size {
        panic!("Invalid Key Settings. Num_Keys must be less than Key_size");
    }
    println!("Settings:");
    println!("  Number of keys: {num_keys}\n  Key size: {key_size}\n  Number of bits per key {num_bits}\n");

    let mut bits = vec![];
    for i in 0..key_size {
        bits.push(Integer::from(2).pow(i));
    }
    bits.shuffle(&mut rng);

    let mut keys = vec![];

    for _key_i in 0..num_keys {
        let mut key = Integer::from(0);
        for _bit_i in 0..num_bits {
            key |= bits.pop().unwrap()
        }
        keys.push(key);
    }
    println!("Raw Keys:");
    for key in keys.clone() {
        println!("  {key}");
    }
    println!("Keys Binary:");
    for key in keys.clone() {
        let mut bin_repr = format!("{key:b}");
        while bin_repr.len() < key_size as usize {
            bin_repr = "0".to_string() + &*bin_repr;
        }
        println!("  {bin_repr}");
    }
    println!("Keys enum:");
    let mut key_keys = HashSet::new();
    while key_keys.len() < keys.len() {
        key_keys.insert(rng.gen());
    }
    let key_keys = key_keys.into_iter().collect::<Vec<u32>>();
    for i in 0..keys.len() {
        println!("  {}: {}", key_keys[i], keys[i]);
    }
}
