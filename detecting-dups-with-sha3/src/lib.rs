use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use sha3::Digest;
use sha3::Sha3_256;
use std::collections::HashMap;

static PHRASE: [&str; 10] = [
    "the quick brown fox jumps over the lazy dog",
    "hello world",
    "rust programming language",
    "data engineering with rust",
    "cryptography and security",
    "machine learning algorithms",
    "artificial intelligence",
    "blockchain technology",
    "distributed systems",
    "concurrency in rust",
];

pub fn generate_random_phrase() -> Vec<&'static str> {
    let mut rng = thread_rng();
    let mut phrases = Vec::new();

    for &phrase in PHRASE.iter() {
        let copies = rng.gen_range(1..=3);
        for _ in 0..copies {
            phrases.push(phrase);
        }
    }
    phrases.shuffle(&mut rng);
    phrases
}

pub fn analyze_duplicates(phrases: &[&str]) {
    let mut hashes: HashMap<_, (usize, &str)> = HashMap::new();
    println!("Total number of phrases: {}", phrases.len());

    for phrase in phrases {
        let hash = Sha3_256::digest(phrase.as_bytes());
        let entry = hashes.entry(hash).or_insert((0, phrase));
        entry.0 += 1;
    }

    let total_unique_phrases = hashes.len();

    let mut total_unique_duplicates = 0;
    let mut total_combined_duplicates = 0;

    for (hash, (count, phrase)) in &hashes {
        if *count > 1 {
            total_unique_duplicates += 1;
            total_combined_duplicates += count - 1;
            println!(
                "Duplicate found: '{}' with hash {:?} appears {} times",
                phrase, hash, count
            );
        }
    }

    println!("Total Unique Phrases: {}", total_unique_phrases);
    println!("Total Unique Duplicates: {}", total_unique_duplicates);
    println!("Total Combined Duplicates: {}", total_combined_duplicates);
}
