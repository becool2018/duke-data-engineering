use std::collections::{BTreeMap, HashMap, LinkedList};
use std::env;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
enum MapKind {
    Hash,
    BTree,
    Vec,
    List,
}

impl MapKind {
    fn from_arg(arg: &str) -> Option<Self> {
        match arg {
            "hash" => Some(Self::Hash),
            "btree" => Some(Self::BTree),
            "vec" => Some(Self::Vec),
            "list" => Some(Self::List),
            _ => None,
        }
    }
}

fn parse_map_kind() -> MapKind {
    for arg in env::args().skip(1) {
        if let Some(value) = arg.strip_prefix("--map=") {
            if let Some(kind) = MapKind::from_arg(value) {
                return kind;
            }
        }
    }
    MapKind::Hash
}

struct BenchResult {
    insert: Duration,
    lookup: Duration,
    hits: usize,
    items: Vec<(String, usize)>,
}

fn main() -> std::io::Result<()> {
    // Read the entire input file into a single string.
    let text = fs::read_to_string("./tenkwords.txt")?;
    // Pre-normalize words so timing focuses on HashMap operations.
    let words: Vec<String> = text
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();

    let map_kind = parse_map_kind();
    let result = match map_kind {
        MapKind::Hash => bench_hashmap(&words),
        MapKind::BTree => bench_btreemap(&words),
        MapKind::Vec => bench_vec(&words),
        MapKind::List => bench_list(&words),
    };

    println!("map: {:?}", map_kind);
    println!("insert: {:?}", result.insert);
    println!("lookup: {:?}", result.lookup);
    println!("hits: {}", result.hits);

    // Print word frequencies, sorted by count desc then word asc.
    let mut items = result.items;
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (word, count) in items {
        println!("{word}: {count}");
    }

    Ok(())
}

fn bench_hashmap(words: &[String]) -> BenchResult {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let insert_start = Instant::now();
    for word in words {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }
    let insert = insert_start.elapsed();

    let lookup_start = Instant::now();
    let mut hits = 0usize;
    for word in words {
        if let Some(count) = counts.get(word) {
            hits += *count;
        }
    }
    let lookup = lookup_start.elapsed();

    let items = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    BenchResult {
        insert,
        lookup,
        hits,
        items,
    }
}

fn bench_btreemap(words: &[String]) -> BenchResult {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let insert_start = Instant::now();
    for word in words {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }
    let insert = insert_start.elapsed();

    let lookup_start = Instant::now();
    let mut hits = 0usize;
    for word in words {
        if let Some(count) = counts.get(word) {
            hits += *count;
        }
    }
    let lookup = lookup_start.elapsed();

    let items = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    BenchResult {
        insert,
        lookup,
        hits,
        items,
    }
}

fn bench_vec(words: &[String]) -> BenchResult {
    let mut counts: Vec<(String, usize)> = Vec::new();
    let insert_start = Instant::now();
    for word in words {
        if let Some((_, count)) = counts.iter_mut().find(|(w, _)| w == word) {
            *count += 1;
        } else {
            counts.push((word.clone(), 1));
        }
    }
    let insert = insert_start.elapsed();

    let lookup_start = Instant::now();
    let mut hits = 0usize;
    for word in words {
        if let Some((_, count)) = counts.iter().find(|(w, _)| w == word) {
            hits += *count;
        }
    }
    let lookup = lookup_start.elapsed();

    BenchResult {
        insert,
        lookup,
        hits,
        items: counts,
    }
}

fn bench_list(words: &[String]) -> BenchResult {
    let mut counts: LinkedList<(String, usize)> = LinkedList::new();
    let insert_start = Instant::now();
    for word in words {
        let mut found = false;
        for (existing, count) in counts.iter_mut() {
            if existing == word {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push_back((word.clone(), 1));
        }
    }
    let insert = insert_start.elapsed();

    let lookup_start = Instant::now();
    let mut hits = 0usize;
    for word in words {
        if let Some((_, count)) = counts.iter().find(|(w, _)| w == word) {
            hits += *count;
        }
    }
    let lookup = lookup_start.elapsed();

    let items = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    BenchResult {
        insert,
        lookup,
        hits,
        items,
    }
}
