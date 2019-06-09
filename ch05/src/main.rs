use std::env;
use std::fs::File;
use std::io::prelude::*;

use rand_os::OsRng;
use rand_os::rand_core::RngCore;

use sha2::{Sha256, Digest};

#[derive(Debug)]
pub enum MnemonicLength {
    L12,
    L15,
    L18,
    L21,
    L24,
}

impl MnemonicLength {
    fn sequence_len(&self) -> usize {
        match self {
            MnemonicLength::L12 => 128 / 8,
            MnemonicLength::L15 => 160 / 8,
            MnemonicLength::L18 => 192 / 8,
            MnemonicLength::L21 => 224 / 8,
            MnemonicLength::L24 => 256 / 8,
        }
    }
}

fn get_sequence<T: RngCore>(mlen: MnemonicLength, rng: &mut T) -> Vec<u8> {
    let mut sequence = vec![0u8; mlen.sequence_len()];
    rng.fill_bytes(&mut sequence);

    sequence
}

fn get_checksum(sequence: &[u8]) -> String {
    let hash = Sha256::digest(sequence);

    let first_byte = hash[0];
    let first_bits = format!("{:08b}", first_byte);
    let checksum_len = sequence.len() * 8 / 32;

    first_bits[0..checksum_len].to_string()
}

fn generate_mnemonic_words(sequence: &[u8], wordlist: &Vec<&str>) -> Vec<String> {
    let checksum = get_checksum(sequence);

    let mut entropy_checksum = sequence
        .iter()
        .fold(String::with_capacity(256), |mut acc, b| {
            acc.push_str(&format!("{:08b}", b));
            acc
        });

    entropy_checksum.push_str(&checksum);

    let mut mnemonic_words = vec![];

    let word_index_bits = 11;
    let word_count = entropy_checksum.len() / word_index_bits;
    for i in 0..word_count {
        let from = i * word_index_bits;
        let to = from + word_index_bits;
        let word_index = usize::from_str_radix(&entropy_checksum[from..to], 2).unwrap();
        mnemonic_words.push(wordlist[word_index].to_string());
    }

    mnemonic_words
}

fn load_wordlist(wordlist_file: &str) -> String {
    let mut file = match File::open(wordlist_file) {
        Err(e) => panic!("Couldn't open wordlist file: {:?}", e),
        Ok(f) => f,
    };

    let mut wordlist = String::new();
    file.read_to_string(&mut wordlist).unwrap();

    wordlist
}



fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Must have exactly 1 argument: path to wordlist file");
    }

    let wordlist_file = &args[0];
    let wordlist = load_wordlist(wordlist_file);
    let wordlist = wordlist.lines().collect();

    let mut os_rng = OsRng::new().unwrap();
    let sequence = get_sequence(MnemonicLength::L15, &mut os_rng);

    let mnemonic_words = generate_mnemonic_words(&sequence, &wordlist);

    for word in mnemonic_words {
        println!("{:?}", word);
    }
}
