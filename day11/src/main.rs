extern crate utils;

use utils::{iter_product, iter_dep_product};

const GRID_SIZE: usize = 300;
const SERIAL_NUMBER: i32 = 6042;

fn power_level(x: usize, y: usize, serial_number: i32) -> i32 {
    let rack_id = x as i32 + 10;
    (((rack_id * y as i32 + serial_number) * rack_id) % 1000) / 100 - 5
}

fn optimize_power(serial_number: i32) -> (usize, (usize, usize)) {
    let mut power_grid = [[0; GRID_SIZE + 1]; GRID_SIZE + 1];

    for (x, y) in iter_product(1..=GRID_SIZE, 1..=GRID_SIZE) {
        power_grid[x][y] = power_level(x, y, serial_number);
    }

    iter_dep_product(1..GRID_SIZE, |square_size| {
        iter_product(1..GRID_SIZE - square_size, 1..GRID_SIZE - square_size)
    })
    .max_by_key(|&(square_size, (xc, yc))| {
        iter_product(xc..xc + square_size, yc..yc + square_size)
            .map(|(x, y)| power_grid[x][y])
            .sum::<i32>()
    })
    .unwrap()
}

fn main() {
    let (size, (opt_x, opt_y)) = optimize_power(SERIAL_NUMBER);
    println!(
        "The optimal power can be found at the location {},{},{} (x, y, size).",
        opt_x, opt_y, size
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_tests() {
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn optimize_power_tests() {
        // assert_eq!((33, 45), optimize_power(18));
        // assert_eq!((21, 61), optimize_power(42));
    }
}
