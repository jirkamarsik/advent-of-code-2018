use std::fmt;
use std::io::Read;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

struct Slice<A> {
    vec: Vec<A>,
    begin: isize,
}

type Rules = [bool; 32];
type SegmentType = usize;

fn segment_type(segment: [bool; 5]) -> SegmentType {
    let mut segment_type = 0;
    for (i, &b) in segment.iter().enumerate() {
        if b {
            segment_type |= 1 << i;
        }
    }
    segment_type
}

fn parse_segment_type(segment_type_str: &str) -> SegmentType {
    let mut segment = [false; 5];
    for (i, b) in segment_type_str.bytes().map(|b| b == b'#').enumerate() {
        segment[i] = b;
    }
    segment_type(segment)
}

struct SlicePositionsIter<'a> {
    slice: &'a Slice<bool>,
    idx: isize,
}

impl<'a> Iterator for SlicePositionsIter<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.slice.end() {
            if self.slice[self.idx] {
                self.idx += 1;
                return Some(self.idx - 1);
            } else {
                self.idx += 1;
            }
        }
        None
    }
}

impl<A> FromIterator<A> for Slice<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        Slice {
            vec: iter.into_iter().collect(),
            begin: 0,
        }
    }
}

impl<A> Slice<A>
where
    A: Copy + Default,
{
    pub fn new(begin: isize, end: isize) -> Slice<A> {
        assert!(begin <= end);
        Slice {
            vec: vec![A::default(); (end - begin) as usize],
            begin,
        }
    }
}

impl<A> Slice<A> {
    fn begin(&self) -> isize {
        self.begin
    }

    fn end(&self) -> isize {
        self.begin + self.vec.len() as isize
    }

    fn shift(&mut self, delta: isize) {
        self.begin += delta;
    }
}

impl Slice<bool> {
    pub fn active_positions(&self) -> SlicePositionsIter {
        SlicePositionsIter {
            slice: self,
            idx: self.begin(),
        }
    }

    fn segment_around(&self, index: isize) -> SegmentType {
        segment_type([
            self[index - 2],
            self[index - 1],
            self[index],
            self[index + 1],
            self[index + 2],
        ])
    }
}

impl Index<isize> for Slice<bool> {
    type Output = bool;

    fn index(&self, index: isize) -> &Self::Output {
        if index >= self.begin() && index < self.end() {
            &self.vec[(index - self.begin) as usize]
        } else {
            &false
        }
    }
}

impl IndexMut<isize> for Slice<bool> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        assert!(index >= self.begin() && index < self.end());
        &mut self.vec[(index - self.begin) as usize]
    }
}

fn parse_input() -> (Slice<bool>, Rules) {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let initial_slice = buffer
        .lines()
        .next()
        .unwrap()
        .trim()
        .bytes()
        .skip("initial state: ".bytes().count())
        .map(|b| b == b'#')
        .collect();

    let mut rules = [false; 32];
    for line in buffer.trim().lines().skip(2) {
        let line = line.trim();
        let segment_type = parse_segment_type(&line[..5]);
        let rhs = line.bytes().last().unwrap() == b'#';
        rules[segment_type] = rhs;
    }

    (initial_slice, rules)
}

impl fmt::Display for Slice<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.begin())?;
        for i in self.begin()..self.end() {
            write!(f, "{}", if self[i] { '#' } else { '.' })?;
        }
        Ok(())
    }
}

fn simulate(mut slice: Slice<bool>, rules: &Rules, iterations: u64) -> Slice<bool> {
    for i in 0..iterations {
        let new_begin = slice.active_positions().min().unwrap() - 2;
        let new_end = slice.active_positions().max().unwrap() + 3;
        let mut new_slice = Slice::new(new_begin, new_end);
        for p in new_slice.begin()..new_slice.end() {
            new_slice[p] = rules[slice.segment_around(p)];
        }
        if slice.vec == new_slice.vec {
            let delta = new_slice.begin() - slice.begin();
            let iterations_done = i + 1;
            let iterations_left = iterations - iterations_done;
            new_slice.shift(iterations_left as isize * delta);
            return new_slice;
        }
        std::mem::swap(&mut slice, &mut new_slice);
    }

    slice
}

fn main() {
    let (initial_state, rules) = parse_input();
    let state_after_twenty = simulate(initial_state, &rules, 20);
    println!(
        "The sum of the numbers of pots with plants after 20 generations is {}.",
        state_after_twenty.active_positions().sum::<isize>()
    );
    let final_state = simulate(state_after_twenty, &rules, 50_000_000_000 - 20);
    println!(
        "The sum of the numbers of pots with plants after 50000000000 generations is {}.",
        final_state.active_positions().sum::<isize>()
    );
}
