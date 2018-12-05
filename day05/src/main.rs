use std::collections::HashMap;
use std::io::Read;

fn parse_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn is_opposite_unit(a: &char, b: &char) -> bool {
    (&a.to_ascii_uppercase() == b && &b.to_ascii_lowercase() == a)
        || (&a.to_ascii_lowercase() == b && &b.to_ascii_uppercase() == a)
}

fn reduce_polymer<I>(polymer: I, capacity: usize) -> usize
where
    I: Iterator<Item = char>,
{
    let mut unit_stack: Vec<char> = Vec::with_capacity(capacity);

    for unit in polymer {
        match unit_stack.pop() {
            Some(opposite_unit) if is_opposite_unit(&unit, &opposite_unit) => (),
            Some(other_unit) => {
                unit_stack.push(other_unit);
                unit_stack.push(unit);
            }
            None => unit_stack.push(unit),
        }
    }

    unit_stack.len()
}

fn kill_unit<I>(polymer: I, lower_unit: char, upper_unit: char) -> impl Iterator<Item = char>
where I: Iterator<Item = char>
{
    polymer.filter(move |&unit| unit != lower_unit && unit != upper_unit)
}

fn find_problematic_unit(polymer: &String) -> (String, usize) {
    let mut scores = HashMap::new();
    for upper_unit in b'A'..=b'Z' {
        // How to trick the compiler in inferring the type 'u8' for 'upper_unit'?
        let upper_unit = upper_unit as char;
        let lower_unit = upper_unit.to_ascii_lowercase();
        let clean_polymer = kill_unit(polymer.chars(), lower_unit, upper_unit);
        let final_length = reduce_polymer(clean_polymer, polymer.len());
        scores.insert(format!("{}/{}", upper_unit, lower_unit), final_length);
    }
    let (unit, &length) = scores.iter().min_by_key(|&(_, &score)| score).unwrap();
    (unit.clone(), length)
}

fn main() {
    let polymer = parse_input();
    println!(
        "The resulting polymer is {} units long!",
        reduce_polymer(polymer.chars(), polymer.len())
    );
    let (problematic_unit, length) = find_problematic_unit(&polymer);
    println!(
        "After removing the problematic unit {}, the resulting polymer is {} units long!",
        problematic_unit, length
    );
}
