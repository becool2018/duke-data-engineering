use sha3_dup_detector::generate_random_phrase;

fn main() {
    let phrases = generate_random_phrase();
    sha3_dup_detector::analyze_duplicates(&phrases);
}
