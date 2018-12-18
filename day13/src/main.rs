use nalgebra as na;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::Read;

#[derive(Clone, Copy)]
enum IntersectionChoice {
    GoLeft,
    GoStraight,
    GoRight,
}

impl IntersectionChoice {
    fn next(self) -> Self {
        match self {
            IntersectionChoice::GoLeft => IntersectionChoice::GoStraight,
            IntersectionChoice::GoStraight => IntersectionChoice::GoRight,
            IntersectionChoice::GoRight => IntersectionChoice::GoLeft,
        }
    }

    fn to_matrix(self) -> na::Matrix2<i32> {
        match self {
            IntersectionChoice::GoLeft => na::Matrix2::new(0, 1, -1, 0),
            IntersectionChoice::GoStraight => na::Matrix2::new(1, 0, 0, 1),
            IntersectionChoice::GoRight => na::Matrix2::new(0, -1, 1, 0),
        }
    }
}

#[derive(Clone, Copy)]
struct Cart {
    pos: na::Point2<i32>,
    dir: na::Vector2<i32>,
    next_intersection: IntersectionChoice,
}

impl Cart {
    fn update(&mut self, map: &Map) {
        self.pos += self.dir;
        match map[self.pos.y as usize][self.pos.x as usize] {
            Tile::StraightPath => (),
            Tile::Curve(transform) => {
                self.dir = transform * self.dir;
            }
            Tile::Intersection => {
                let transform = self.next_intersection.to_matrix();
                self.dir = transform * self.dir;
                self.next_intersection = self.next_intersection.next();
            }
            Tile::Empty => {
                panic!("A cart has gone off the tracks!");
            }
        }
    }
}

struct CartInSimulation(Cart);

impl PartialEq for CartInSimulation {
    fn eq(&self, other: &CartInSimulation) -> bool {
        self.0.pos.eq(&other.0.pos)
    }
}

impl Eq for CartInSimulation {}

impl PartialOrd for CartInSimulation {
    fn partial_cmp(&self, other: &CartInSimulation) -> Option<Ordering> {
        let y_cmp = self.0.pos.y.cmp(&other.0.pos.y).reverse();
        let x_cmp = self.0.pos.x.cmp(&other.0.pos.x).reverse();
        Some(y_cmp.then(x_cmp))
    }
}

impl Ord for CartInSimulation {
    fn cmp(&self, other: &CartInSimulation) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

enum Tile {
    StraightPath,
    Curve(na::Matrix2<i32>),
    Intersection,
    Empty,
}

type Map = Vec<Vec<Tile>>;

fn parse_input() -> (Map, Vec<Cart>) {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut carts = Vec::new();
    let mut map = Vec::new();

    for (y, line) in buffer.lines().enumerate() {
        map.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            map[y].push(match c {
                '|' | '-' => Tile::StraightPath,
                '/' => Tile::Curve(na::Matrix2::new(0, -1, -1, 0)),
                '\\' => Tile::Curve(na::Matrix2::new(0, 1, 1, 0)),
                '+' => Tile::Intersection,
                ' ' => Tile::Empty,
                '^' | 'v' | '<' | '>' => {
                    carts.push(Cart {
                        pos: na::Point2::new(x as i32, y as i32),
                        dir: match c {
                            '^' => na::Vector2::new(0, -1),
                            'v' => na::Vector2::new(0, 1),
                            '<' => na::Vector2::new(-1, 0),
                            '>' => na::Vector2::new(1, 0),
                            _ => panic!("Impossible to reach."),
                        },
                        next_intersection: IntersectionChoice::GoLeft,
                    });
                    Tile::StraightPath
                }
                _ => panic!("Unexpected character in input!"),
            });
        }
    }

    (map, carts)
}

fn simulate_until_first_crash(map: &Map, carts: &[Cart]) -> na::Point2<i32> {
    let mut queue = carts
        .iter()
        .map(|&cart| CartInSimulation(cart))
        .collect::<BinaryHeap<CartInSimulation>>();
    let mut next_queue = BinaryHeap::new();
    let mut occupied = carts
        .iter()
        .map(|cart| cart.pos)
        .collect::<HashSet<na::Point2<i32>>>();

    loop {
        while let Some(CartInSimulation(mut cart)) = queue.pop() {
            occupied.remove(&cart.pos);
            cart.update(&map);
            if !occupied.insert(cart.pos) {
                return cart.pos;
            }
            next_queue.push(CartInSimulation(cart));
        }
        std::mem::swap(&mut queue, &mut next_queue);
    }
}

fn remove_from_heap(
    heap: BinaryHeap<CartInSimulation>,
    pos: na::Point2<i32>,
) -> BinaryHeap<CartInSimulation> {
    heap.into_iter()
        .filter(|CartInSimulation(cart2)| pos != cart2.pos)
        .collect()
}

fn simulate_until_last_cart(map: &Map, carts: &[Cart]) -> na::Point2<i32> {
    let mut queue = carts
        .iter()
        .map(|&cart| CartInSimulation(cart))
        .collect::<BinaryHeap<CartInSimulation>>();
    let mut next_queue = BinaryHeap::new();
    let mut occupied = carts
        .iter()
        .map(|cart| cart.pos)
        .collect::<HashSet<na::Point2<i32>>>();

    loop {
        if queue.len() == 1 {
            return queue.pop().unwrap().0.pos;
        }
        while let Some(CartInSimulation(mut cart)) = queue.pop() {
            occupied.remove(&cart.pos);
            cart.update(&map);
            if !occupied.insert(cart.pos) {
                occupied.remove(&cart.pos);
                queue = remove_from_heap(queue, cart.pos);
                next_queue = remove_from_heap(next_queue, cart.pos);
            } else {
                next_queue.push(CartInSimulation(cart));
            }
        }
        std::mem::swap(&mut queue, &mut next_queue);
    }
}

fn wait_on_input() {
    let mut dummy_buffer = String::new();
    std::io::stdin().read_line(&mut dummy_buffer).unwrap();
    print!("{control}[2J", control = 27 as char);
}

fn bold_red(text: &str) -> String {
    format!("{control}[1;31m{}{control}[0m", text, control = 27 as char)
}

fn print_state(map: &Map, carts: &[Cart]) {
    let UP = na::Vector2::new(0, -1);
    let DOWN = na::Vector2::new(0, 1);
    let LEFT = na::Vector2::new(-1, 0);
    let RIGHT = na::Vector2::new(1, 0);
    let CURVE_A = &na::Matrix2::new(0, -1, -1, 0);
    let CURVE_B = &na::Matrix2::new(0, 1, 1, 0);

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let s = if let Some(cart) = carts
                .iter()
                .find(|cart| cart.pos == na::Point2::new(x as i32, y as i32))
            {
                if cart.dir == UP {
                    bold_red("^")
                } else if cart.dir == DOWN {
                    bold_red("v")
                } else if cart.dir == LEFT {
                    bold_red("<")
                } else if cart.dir == RIGHT {
                    bold_red(">")
                } else {
                    panic!("Unexpected direction of cart!")
                }
            } else {
                match tile {
                    Tile::StraightPath => ".".to_owned(),
                    Tile::Curve(transform) => {
                        if transform == CURVE_A {
                            "/".to_owned()
                        } else if transform == CURVE_B {
                            "\\".to_owned()
                        } else {
                            panic!("Unexpected transform in curve tile data.")
                        }
                    }
                    Tile::Intersection => "+".to_owned(),
                    Tile::Empty => " ".to_owned(),
                }
            };
            print!("{}", s);
        }
        println!();
    }
}

fn main() {
    let (map, carts) = parse_input();
    let crash = simulate_until_first_crash(&map, &carts);
    println!(
        "The first crash will occur at position: {},{}",
        crash.x, crash.y
    );
    let last = simulate_until_last_cart(&map, &carts);
    println!("The last cart will be at position: {},{}", last.x, last.y);
}
