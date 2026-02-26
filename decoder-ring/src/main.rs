use clap::Parser;
use decoder_ring::print_stats_analysis;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to decrypt
    #[arg(short, long)]
    message: String,

    //statistical information about the message
    #[arg(short, long)]
    stats: bool,

    //guess the shift
    #[arg(short, long)]
    guess: bool,
}

fn main() {
    let args = Args::parse();

    if args.stats {
        print_stats_analysis(&args.message);
    }

    if args.guess {
        // Implement shift guessing logic here
        let (depth, best_shift, decrypted, max_score) =
            decoder_ring::guess_shift(&args.message, 27);
        println!("Best shift: {}", best_shift);
        println!("Decrypted message: {}", decrypted);
        println!("Max score: {}", max_score);
        println!("Depth: {}", depth);
    }
}
