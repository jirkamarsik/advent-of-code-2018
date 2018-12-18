use aho_corasick::{AcAutomaton, Automaton};
use std::fmt::Write;
use std::io;
use std::io::Read;

const PUZZLE_INPUT: usize = 293801;

struct KitchenState {
    recipes: Vec<u8>,
    first_cook: usize,
    second_cook: usize,
}

impl KitchenState {
    fn new() -> KitchenState {
        Self::with_capacity(1000)
    }

    fn with_capacity(capacity: usize) -> KitchenState {
        let mut recipes = Vec::with_capacity(capacity);
        recipes.push(3);
        recipes.push(7);
        KitchenState {
            recipes,
            first_cook: 0,
            second_cook: 1,
        }
    }

    fn cook(&mut self) {
        let first_score = self.recipes[self.first_cook];
        let second_score = self.recipes[self.second_cook];
        let score_sum = first_score + second_score;
        let tens_digit = score_sum / 10;
        let ones_digit = score_sum % 10;
        if tens_digit > 0 {
            self.recipes.push(tens_digit);
        }
        self.recipes.push(ones_digit);
        self.first_cook = (self.first_cook + first_score as usize + 1) % self.recipes.len();
        self.second_cook = (self.second_cook + second_score as usize + 1) % self.recipes.len();
    }

    fn to_reader(self) -> KitchenReader {
        KitchenReader {
            kitchen_state: self,
            produced: 0,
        }
    }
}

struct KitchenReader {
    kitchen_state: KitchenState,
    produced: usize,
}

impl Read for KitchenReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = buf.len();
        while self.kitchen_state.recipes.len() < self.produced + n {
            self.kitchen_state.cook();
        }
        buf.copy_from_slice(&self.kitchen_state.recipes[self.produced..self.produced + n]);
        self.produced += n;
        Ok(n)
    }
}

fn find_10_recipes_at_offset(offset: usize) -> String {
    let mut kitchen_state = KitchenState::with_capacity(offset + 10 + 1);
    while kitchen_state.recipes.len() < offset + 10 {
        kitchen_state.cook();
    }
    let mut score_string = String::with_capacity(10);
    for score in kitchen_state.recipes[offset..offset + 10].iter() {
        write!(&mut score_string, "{}", score).unwrap();
    }
    score_string
}

fn puzzle_input_to_score_sequence(input: usize) -> Vec<u8> {
    format!("{}", input)
        .chars()
        .map(|c| {
            let mut string = String::with_capacity(1);
            string.push(c);
            string.parse().unwrap()
        })
        .collect()
}

fn find_index_of_recipe_scores<P>(score_sequence: P) -> Option<usize>
where
    P: AsRef<[u8]>,
{
    let patterns = [score_sequence];
    let automaton = AcAutomaton::new(&patterns);
    automaton
        .stream_find(KitchenState::new().to_reader())
        .next()
        .map(|m| m.unwrap().start)
}

fn main() {
    println!(
        "The 10 recipes produced after the first {} recipes have the scores: {}",
        PUZZLE_INPUT,
        find_10_recipes_at_offset(PUZZLE_INPUT)
    );
    match find_index_of_recipe_scores(puzzle_input_to_score_sequence(PUZZLE_INPUT)) {
        Some(i) => println!("{} recipes appear to the left of the score sequnce.", i),
        None => println!("The score sequence never occurs!"),
    }
}
