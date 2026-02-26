use ceaser_cipher::decrypt;
use ceaser_cipher::encrypt;

fn main() {
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let shift = 3;
    println!("Plaintext: {}", plaintext);
    let ciphertext = encrypt(plaintext, shift);
    println!("Ciphertext: {}", ciphertext);

    let decrypted_text = decrypt(&ciphertext, shift);
    println!("Decrypted text: {}", decrypted_text);
}
