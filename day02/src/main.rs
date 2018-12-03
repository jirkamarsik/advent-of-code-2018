use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn parse_input() -> Vec<String> {
    let mut codes = vec![];

    let handle = io::stdin();
    for line in handle.lock().lines() {
        codes.push(line.unwrap().trim().to_owned());
    }

    codes
}

fn frequencies(text: &str) -> HashMap<char, u32> {
    let mut freqs = HashMap::with_capacity(text.len());

    for c in text.chars() {
        let freq = freqs.entry(c).or_insert(0);
        *freq += 1;
    }

    return freqs
}

fn checksum(codes: &Vec<String>) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;

    for code in codes {
        let freqs = frequencies(code);
        if freqs.values().find(|f| **f == 2).is_some() {
            doubles += 1;
        }
        if freqs.values().find(|f| **f == 3).is_some() {
            triples += 1;
        }
    }

    doubles * triples
}

fn main() {
    let codes = parse_input();
    println!("The checksum is {}.", checksum(&codes));
}
