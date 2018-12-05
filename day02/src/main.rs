use std::collections::HashMap;
use std::hash::Hash;
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

fn frequencies<T>(seq: T) -> HashMap<T::Item, u32>
where T: Iterator,
      T::Item: Eq + Hash
{
    let mut freqs = HashMap::new();

    for x in seq {
        let freq = freqs.entry(x).or_insert(0);
        *freq += 1;
    }

    freqs
}

fn checksum(codes: &[String]) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;

    for code in codes {
        let freqs = frequencies(code.chars());
        if freqs.values().any(|&f| f == 2) {
            doubles += 1;
        }
        if freqs.values().any(|&f| f == 3) {
            triples += 1;
        }
    }

    doubles * triples
}

fn find_boxes(codes: &[String]) -> Option<String> {
    let code_length = codes[0].len();
    assert!(codes.iter().all(|code| code.len() == code_length));

    for i in 0..code_length {
        let freqs = frequencies(codes.iter().map(|code| (&code[..i], &code[i+1..])));
        if let Some(((left, right), _)) = freqs.iter().find(|&(_, &freq)| freq >= 2) {
            return Some(format!("{}{}", left, right));
        }
    }

    None
}

fn main() {
    let codes = parse_input();
    println!("The checksum is {}.", checksum(&codes));
    match find_boxes(&codes) {
        Some(common_part) => println!("Matching boxes found with common substring '{}'.", common_part),
        None => println!("No matching boxes found!"),
    }
}
