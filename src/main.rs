use core::str;
use std::io::BufRead;

use rand::seq::SliceRandom;
use rand::thread_rng;
use unicode_segmentation::UnicodeSegmentation;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn alphabet_permutation() -> String {
    let mut graphemes = ALPHABET.graphemes(true).collect::<Vec<&str>>();
    let gslice = graphemes.as_mut_slice();
    let mut rng = thread_rng();
    gslice.shuffle(&mut rng);
    gslice.iter().copied().collect::<String>()
}

fn scramble_text(text: &str, original: &str, permutation: &str) -> String {
    text.chars()
        .map(|c| {
            if let Some(z) = original.find(c.to_ascii_uppercase()) {
                let nc = permutation.as_bytes()[z] as char;
                if c.is_ascii_uppercase() {
                    nc
                } else {
                    nc.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect::<String>()
}

fn main() {
    let permutation = alphabet_permutation();

    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_until(26, &mut input);

    let translated = scramble_text(
        String::from_utf8(input)
            .expect("Input bytes could not be decoded to UTF-8.")
            .as_str(),
        ALPHABET,
        &permutation,
    );
    print!("{translated}");
}
