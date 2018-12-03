extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

struct Rect {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

struct Range {
    from: i32,
    to: i32,
}

struct Claim {
    id: i32,
    rect: Rect,
}

fn parse_input() -> Vec<Claim> {
    let mut claims = vec![];

    let line_parser =
        Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)")
            .unwrap();

    let handle = io::stdin();
    for line in handle.lock().lines() {
        let line = line.unwrap();
        let caps = line_parser.captures(&line).expect("Parse error!");
        claims.push(Claim {
            id: caps["id"].parse().unwrap(),
            rect: Rect {
                left: caps["left"].parse().unwrap(),
                top: caps["top"].parse().unwrap(),
                width: caps["width"].parse().unwrap(),
                height: caps["height"].parse().unwrap(),
            },
        });
    }

    claims
}

fn horizontal_proj(rect: &Rect) -> Range {
    Range {
        from: rect.left,
        to: rect.left + rect.width,
    }
}

fn vertical_proj(rect: &Rect) -> Range {
    Range {
        from: rect.top,
        to: rect.top + rect.height,
    }
}

fn is_intersect_rect(a: &Rect, b: &Rect) -> bool {
    is_intersect_range(&horizontal_proj(a), &horizontal_proj(b))
        && is_intersect_range(&vertical_proj(a), &vertical_proj(b))
}

fn is_intersect_range(a: &Range, b: &Range) -> bool {
    !(a.to <= b.from || a.from >= b.to)
}

fn rect_intersection(a: &Rect, b: &Rect) -> Option<Rect> {
    if is_intersect_rect(a, b) {
        let left = a.left.max(b.left);
        let right = (a.left + a.width).min(b.left + b.width);
        let top = a.top.max(b.top);
        let bottom = (a.top + a.height).min(b.top + b.height);
        Some(Rect {
            left,
            top,
            width: right - left,
            height: bottom - top,
        })
    } else {
        None
    }
}

fn intersections(claims: &Vec<Claim>) -> Vec<Rect> {
    let mut intersections = vec![];

    for i in 0..claims.len() - 1 {
        for j in i + 1..claims.len() {
            if let Some(r) = rect_intersection(&claims[i].rect, &claims[j].rect) {
                intersections.push(r);
            }
        }
    }

    intersections
}

fn contested_inches(claims: &Vec<Claim>) -> HashSet<(i32, i32)> {
    let intersections = intersections(claims);
    let mut inches = HashSet::new();

    for rect in intersections {
        for x in rect.left..rect.left + rect.width {
            for y in rect.top..rect.top + rect.height {
                inches.insert((x, y));
            }
        }
    }

    inches
}

fn safe_claim<'a>(claims: &'a Vec<Claim>) -> Option<&'a Claim> {
    claims
        .iter()
        .filter(|Claim { rect: rect1, .. }| {
            claims
                .iter()
                .filter(|Claim { rect: rect2, .. }| is_intersect_rect(rect1, rect2))
                .count()
                == 1
        })
        .next()
}

fn main() {
    let claims = parse_input();
    println!(
        "There are {} square inches of contested fabric.",
        contested_inches(&claims).len()
    );
    match safe_claim(&claims) {
        Some(Claim { id, .. }) => println!("The claim #{} overlaps no other claim.", id),
        None => println!("All the claims overlap!"),
    }
}
