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

fn solve(freq_changes: Vec<i32>) -> i32 {
    let mut frequency = 0;

    for delta in freq_changes {
        frequency += delta;
    }

    frequency
}

fn main() {
    let input = parse_input();
    println!("{}", solve(input));
}
