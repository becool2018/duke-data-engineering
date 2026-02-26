pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let a = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - a + shift) % 26;
            result.push((a + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}
