use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufRead;
use std::path::Path;
use std::{env, io, process};

use rand::seq::SliceRandom;
use rand::thread_rng;
use unicode_segmentation::UnicodeSegmentation;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn alphabet_permutation() -> String {
    let mut graphemes = ALPHABET.graphemes(true).collect::<Vec<&str>>();
    let gslice = graphemes.as_mut_slice();
    let mut rng = thread_rng();
    gslice.shuffle(&mut rng);
    gslice.iter().map(|x| *x).collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Requires 2 positional arguments: input_file output_file");
        process::exit(1);
    }

    let permutation = alphabet_permutation();
    println!("Using permutation: {}", permutation);

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(args[2].clone())
        .expect("Could not create output file.");

    if let Ok(lines) = read_lines(args[1].clone()) {
        for line in lines.flatten() {
            let translated = line
                .to_ascii_uppercase()
                .chars()
                .map(|c| {
                    if let Some(z) = permutation.get(c) {
                        let test = permutation.as_bytes()[z] as char;
                        dbg!(permutation.as_bytes());
                        dbg!(test);
                        test
                    } else {
                        c
                    }
                })
                .collect::<String>();
            writeln!(output_file, "{}", translated).unwrap();
        }
    } else {
        println!("Could not open input file.");
        process::exit(1);
    }
}
