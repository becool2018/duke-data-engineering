use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;
fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
        "Kiwi",
        "Lemon",
        "Mango",
        "Nectarine",
        "Orange",
        "Papaya",
        "Quince",
        "Raspberry",
        "Strawberry",
        "Tangerine",
        "Ugli Fruit",
        "Vanilla Bean",
        "Watermelon",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}
