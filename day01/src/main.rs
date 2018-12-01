use std::io;
use std::io::prelude::*;

fn parse_input() -> Vec<i32> {
    let mut freq_changes = vec![];

    let handle = io::stdin();
    for line in handle.lock().lines() {
        freq_changes.push(line.unwrap().parse().expect("Illegal input"));
    }

    freq_changes
}

fn final_frequency(freq_changes: &Vec<i32>) -> i32 {
    freq_changes.iter().sum()
}

fn repeating_frequency(freq_changes: &Vec<i32>) -> Option<i32> {
    let n = freq_changes.len();
    let mut big_delta = 0;
    let mut partial_deltas = Vec::with_capacity(n);

    for delta in freq_changes {
        partial_deltas.push(big_delta);
        big_delta += delta;
    }

    let mut best_candidate = None;

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let delta = partial_deltas[j] - partial_deltas[i];
                if (delta % big_delta == 0) && (delta / big_delta >= 0) {
                    let new_candidate = (delta / big_delta, i);
                    best_candidate = match best_candidate {
                        None => Some(new_candidate),
                        Some(best_candidate) => Some(best_candidate.min(new_candidate)),
                    };
                }
            }
        }
    }

    best_candidate.map(|(iterations, index)| iterations * big_delta + partial_deltas[index])
}

fn main() {
    let input = parse_input();
    println!("The final frequency is {}.", final_frequency(&input));
    match repeating_frequency(&input) {
        Some(frequency) => println!("The first repeating frequency is {}.", frequency),
        None => println!("The frequencies will never repeat!"),
    }
}
