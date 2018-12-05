use std::collections::HashMap;
use std::io::Read;

fn parse_input() -> Vec<u8> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.trim().as_bytes().to_vec()
}

fn is_opposite_unit(a: u8, b: u8) -> bool {
    let a = a as char;
    let b = b as char;
    (a.to_ascii_uppercase() == b && b.to_ascii_lowercase() == a)
        || (a.to_ascii_lowercase() == b && b.to_ascii_uppercase() == a)
}

fn move_left(alive_map: &[bool], pos: usize) -> Option<usize> {
    for new_pos in (0..pos).rev() {
        if alive_map[new_pos] {
            return Some(new_pos)
        }
    }
    None
}

fn move_right(alive_map: &[bool], pos: usize) -> Option<usize> {
    for new_pos in pos + 1..alive_map.len() {
        if alive_map[new_pos] {
            return Some(new_pos)
        }
    }
    None
}

fn build_alive_map(polymer: &[u8]) -> Vec<bool> {
    let mut alive_map = Vec::with_capacity(polymer.len());
    for _ in 0..polymer.len() {
        alive_map.push(true);
    }
    alive_map
}

fn reduce_polymer(polymer: &[u8], alive_map: &mut [bool]) {
    let mut left = Some(polymer.len() - 2);
    let mut right = Some(polymer.len() - 1);
    while let (Some(l), Some(r)) = (left, right) {
        if is_opposite_unit(polymer[l], polymer[r]) {
            alive_map[l] = false;
            alive_map[r] = false;
            left = move_left(&alive_map, l);
            right = move_right(&alive_map, r);
        } else {
            left = move_left(&alive_map, l);
            right = move_left(&alive_map, r);
        }
    }
}

fn polymer_length(alive_map: &[bool]) -> usize {
    alive_map.iter().filter(|&&live| live).count()
}

fn kill_unit(polymer: &[u8], alive_map: &mut [bool], lower_unit: u8, upper_unit: u8) {
    for (&unit, alive) in polymer.iter().zip(alive_map.iter_mut()) {
        if unit == lower_unit || unit == upper_unit {
            *alive = false;
        }
    }
}

fn reset_polymer(alive_map: &mut [bool]) {
    for alive in alive_map.iter_mut() {
        *alive = true;
    }
}

fn find_problematic_unit(polymer: &[u8], alive_map: &mut [bool]) -> (String, usize) {
    let mut scores = HashMap::new();
    // for upper_unit in 'A'..'Z' {
    for upper_unit in 0x41..0x5B {
        let lower_unit = (upper_unit as char).to_ascii_lowercase() as u8;
        reset_polymer(alive_map);
        kill_unit(polymer, alive_map, lower_unit, upper_unit);
        reduce_polymer(polymer, alive_map);
        scores.insert(
            format!("{}/{}", upper_unit as char, lower_unit as char),
            polymer_length(alive_map),
        );
    }
    let (unit, &length) = scores.iter().min_by_key(|&(_, &score)| score).unwrap();
    (unit.clone(), length)
}

fn main() {
    let polymer = parse_input();
    let mut alive_map = build_alive_map(&polymer);
    reduce_polymer(&polymer, &mut alive_map);
    println!(
        "The resulting polymer is {} units long!",
        polymer_length(&alive_map)
    );
    let (problematic_unit, length) = find_problematic_unit(&polymer, &mut alive_map);
    println!(
        "After removing the problematic unit {}, the resulting polymer is {} units long!",
        problematic_unit, length
    );
}
