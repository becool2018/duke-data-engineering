use std::collections::HashMap;

fn gen_counts() -> HashMap<char, f32> {
    let mut eng_freq: HashMap<char, f32> = HashMap::new();

    eng_freq.insert('a', 12.7);
    eng_freq.insert('b', 1.4);
    eng_freq.insert('c', 2.7);
    eng_freq.insert('d', 4.2);
    eng_freq.insert('e', 12.7);
    eng_freq.insert('f', 2.2);
    eng_freq.insert('g', 2.0);
    eng_freq.insert('h', 6.0);
    eng_freq.insert('i', 6.9);
    eng_freq.insert('j', 14.1);
    eng_freq.insert('k', 15.7);

    eng_freq
}

pub fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();

    let eng_freq_map = gen_counts();
    let eng_freq_map = eng_freq_map
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<HashMap<char, f32>>();

    let mut results = Vec::new();

    for (letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let eng_freq = eng_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let eng_freq_diff = eng_freq.map_or(0.0, |f| (freq - f as f32).abs());

        results.push((*letter, *count, freq, eng_freq, eng_freq_diff));
    }

    results
}

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    println!("Letter | Count | Frequency (%) | English Freq (%) | Diff");
    println!("---------------------------------------------------------");
    for (letter, count, freq, eng_freq, eng_freq_diff) in stats {
        let eng_freq_str = eng_freq
            .map(|f| format!("{:.2}", f))
            .unwrap_or_else(|| "N/A".to_string());
        println!(
            "{:>6} | {:>5} | {:>13.2} | {:>17} | {:.2}",
            letter, count, freq, eng_freq_str, eng_freq_diff
        );
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let a = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - a + 26 - shift) % 26;
            result.push((a + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn guess_shift(text: &str, depth: u8) -> (u8, u8, String, f32) {
    let mut max_score = 0.0;
    let mut best_shift = 0;
    let mut decrypted = String::new();

    for shift in 0..depth {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        let mut score = 0.0;
        for (_, _, freq, eng_freq, eng_freq_diff) in stats {
            if let Some(eng_freq) = eng_freq {
                score += (1.0 - (eng_freq_diff / eng_freq)) * freq;
            }
        }
        println!("Shift: {}, Score: {}", shift, score);
        if score > max_score {
            max_score = score;
            best_shift = shift;
            decrypted = decrypted_text;
        }
    }

    (depth, best_shift, decrypted, max_score)
}
