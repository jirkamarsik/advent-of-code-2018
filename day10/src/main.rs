#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

struct Star {
    position: (i32, i32),
    velocity: (i32, i32),
}

struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<bool>,
}

impl Star {
    fn advance(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

impl FromStr for Star {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Star, Self::Err> {
        lazy_static! {
            static ref PARSER: Regex = Regex::new(
                r"(?x)
position = < \s* (?P<px>-?\d+) , \s* (?P<py>-?\d+) > \s*
velocity = < \s* (?P<vx>-?\d+) , \s* (?P<vy>-?\d+) >"
            )
            .unwrap();
        }

        match PARSER.captures(s) {
            Some(caps) => Ok(Star {
                position: (caps["px"].parse()?, caps["py"].parse()?),
                velocity: (caps["vx"].parse()?, caps["vy"].parse()?),
            }),
            None => Err(Box::from("Could not parse star specification!")),
        }
    }
}

fn parse_input() -> Vec<Star> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|line| line.parse().unwrap()).collect()
}

fn laydown(stars: &[Star]) -> Option<Canvas> {
    let left = stars.iter().map(|star| star.position.0).min().unwrap();
    let right = stars.iter().map(|star| star.position.0).max().unwrap();
    let top = stars.iter().map(|star| star.position.1).min().unwrap();
    let bottom = stars.iter().map(|star| star.position.1).max().unwrap();

    let width = (right - left + 1) as usize;
    let height = (bottom - top + 1) as usize;

    if width * height > 10000 {
        return None;
    }

    let mut buffer = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        buffer.push(false);
    }

    for star in stars {
        let line = (star.position.1 - top) as usize;
        let column = (star.position.0 - left) as usize;
        buffer[line * width + column] = true;
    }

    Some(Canvas {
        width,
        height,
        buffer,
    })
}

fn render(canvas: &Canvas) -> String {
    let mut output = String::with_capacity(canvas.height * (canvas.width + 1));
    for line in 0..canvas.height {
        for column in 0..canvas.width {
            let with_star = canvas.buffer[line * canvas.width + column];
            output.push(if with_star { '#' } else { '.' });
        }
        output.push('\n');
    }
    output
}

fn advance_stars(stars: &mut [Star]) {
    for star in stars {
        star.advance();
    }
}

fn might_contain_text(canvas: &Canvas) -> bool {
    let mut longest_vertical_segment = 0;
    for column in 0..canvas.width {
        let mut current_vertical_segment = 0;
        for line in 0..canvas.height {
            if canvas.buffer[line * canvas.width + column] {
                current_vertical_segment += 1;
            } else {
                longest_vertical_segment = longest_vertical_segment.max(current_vertical_segment);
                current_vertical_segment = 0;
            }
        }
        longest_vertical_segment = longest_vertical_segment.max(current_vertical_segment);
    }

    longest_vertical_segment >= 8
}

fn main() {
    let mut stars = parse_input();
    let mut time = 0;
    loop {
        if let Some(canvas) = laydown(&stars) {
            if might_contain_text(&canvas) {
                println!("{}", render(&canvas));
                println!("This message will appear in {} seconds.", time);
                break;
            }
        }
        advance_stars(&mut stars);
        time += 1;
    }
}
