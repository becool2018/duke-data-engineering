/*
  Usage: cargo run -- fruits.csv
  or
  cargo run -- --fruits "apple, pear"
*/

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;

#[derive(Parser, Debug)]
#[clap(
    name = "Fruit Salad Maker",
    version = "0.1.0",
    author = "Duke Data Engineering",
    about = "Creates a random fruit salad from a list of fruits"
)]

struct Ops {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}
fn display_fruit_salad(fruit_salad: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruit_salad {
        println!("- {}", fruit);
    }
}

fn main() {
    let opts = Ops::parse();

    // Use fruits from CSV file or command-line argument
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Failed to read CSV file");
            csv_to_vec(&fruits)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);
}
