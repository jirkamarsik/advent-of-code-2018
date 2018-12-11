#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[derive(Debug)]
enum PointClass<'a> {
    NearestTo(&'a Point),
    Tied,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        lazy_static! {
            static ref POINT_PARSER: Regex = Regex::new(r"(?P<x>\d+),\s+(?P<y>\d+)").unwrap();
        }

        match POINT_PARSER.captures(s) {
            Some(caps) => Ok(Point {
                x: caps["x"].parse()?,
                y: caps["y"].parse()?,
            }),
            None => Err(Box::from("Cannot parse point!")),
        }
    }
}

impl Point {
    fn neighbors(&self) -> NeighborsIter {
        NeighborsIter {
            point: self,
            index: 0,
        }
    }
}

struct NeighborsIter<'a> {
    point: &'a Point,
    index: u32,
}

impl<'a> Iterator for NeighborsIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(Point {
                x: self.point.x,
                y: self.point.y - 1,
            }),
            2 => Some(Point {
                x: self.point.x + 1,
                y: self.point.y,
            }),
            3 => Some(Point {
                x: self.point.x,
                y: self.point.y + 1,
            }),
            4 => Some(Point {
                x: self.point.x - 1,
                y: self.point.y,
            }),
            _ => None,
        }
    }
}

fn parse_input() -> Result<Vec<Point>, Box<dyn Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    buffer.lines().map(|line| line.parse()).collect()
}

fn distance(a: &Point, b: &Point) -> u32 {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as u32
}

fn classify<'a>(locations: &'a [Point], point: &Point) -> PointClass<'a> {
    let mut locations_with_distances = locations
        .iter()
        .map(|location| (location, distance(point, location)))
        .collect::<Vec<_>>();

    locations_with_distances.sort_by_key(|&(_, dist)| dist);

    if locations_with_distances[0].1 == locations_with_distances[1].1 {
        PointClass::Tied
    } else {
        PointClass::NearestTo(locations_with_distances[0].0)
    }
}

fn in_infinity(bounds: &Bounds, point: &Point) -> bool {
    point.x < bounds.min_x
        || point.x > bounds.max_x
        || point.y < bounds.min_y
        || point.y > bounds.max_y
}

fn explore_generic<F>(start: &Point, should_continue: F) -> Option<u32>
where
    F: Fn(&Point) -> Option<bool>,
{
    let mut scheduled = HashSet::new();
    let mut stack = Vec::new();
    let mut size = 0;

    scheduled.insert(start.clone());
    stack.push(start.clone());

    while let Some(loc) = stack.pop() {
        if should_continue(&loc)? {
            size += 1;
            for neighbor in loc.neighbors() {
                if scheduled.insert(neighbor.clone()) {
                    stack.push(neighbor);
                }
            }
        }
    }

    Some(size)
}

fn explore_part1(bounds: &Bounds, locations: &[Point], start: &Point) -> Option<u32> {
    explore_generic(start, |loc| {
        if let PointClass::NearestTo(n_loc) = classify(locations, loc) {
            if n_loc == start && in_infinity(bounds, loc) {
                None
            } else {
                Some(n_loc == start)
            }
        } else {
            Some(false)
        }
    })
}

fn find_size_safest_zone(locations: &[Point]) -> Option<u32> {
    let bounds = Bounds {
        min_x: locations.iter().map(|p| p.x).min().unwrap(),
        max_x: locations.iter().map(|p| p.x).max().unwrap(),
        min_y: locations.iter().map(|p| p.y).min().unwrap(),
        max_y: locations.iter().map(|p| p.y).max().unwrap(),
    };

    locations
        .iter()
        .filter_map(|start| explore_part1(&bounds, locations, start))
        .max()
}

fn explore_part2(locations: &[Point], limit: u32, start: &Point) -> u32 {
    explore_generic(start, |loc| {
        Some(
            locations
                .iter()
                .map(|location| distance(loc, location))
                .sum::<u32>()
                < limit,
        )
    })
    .unwrap()
}

fn centroid(locations: &[Point]) -> Point {
    let mut centroid = Point { x: 0, y: 0 };
    for point in locations {
        centroid.x += point.x;
        centroid.y += point.y;
    }
    centroid.x /= locations.len() as i32;
    centroid.y /= locations.len() as i32;
    centroid
}

fn find_brave_zone_size(locations: &[Point]) -> u32 {
    explore_part2(locations, 10_000, &centroid(locations))
}

fn main() -> Result<(), Box<dyn Error>> {
    let locations = parse_input()?;
    match find_size_safest_zone(&locations) {
        Some(size) => println!("The safest zone has size {}.", size),
        None => return Err(Box::from("Could not find any safe zone!")),
    }
    println!(
        "The very brave zone has size {}.",
        find_brave_zone_size(&locations)
    );
    Ok(())
}
