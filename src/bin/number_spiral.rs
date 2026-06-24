// https://cses.fi/problemset/task/1071

/*
 *   x  1  2  3  4  5  6  7  8
 * y --------------------------
 * 1 |  1  2  9  10 25 26 49 50
 * 2 |  4  3  8  11 24 27 48 51
 * 3 |  5  6  7  12 23 28 47 52
 * 4 |  16 15 14 13 22 29 46 53
 * 5 |  17 18 19 20 21 30 45 54
 * 6 |  36 35 34 33 32 31 44 55
 * 7 |  37 38 39 40 41 42 43 56
 * 8 |  64 63 62 61 60 59 58 57
 */

use std::io::{BufRead, BufReader};

fn main() {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().unwrap();

    let mut counter: u32 = 0;
    let mut coords_list: Vec<(u64, u64)> = Vec::new();

    while counter < n {
        line.clear();
        input.read_line(&mut line).unwrap();
        let mut entry = line.split_whitespace();
        let y: u64 = entry.next().unwrap().parse().unwrap();
        let x: u64 = entry.next().unwrap().parse().unwrap();

        coords_list.push((y, x));

        counter += 1;
    }

    for (y, x) in coords_list {
        println!("{}", number_at_coords(y, x));
    }
}

fn number_at_coords(y: u64, x: u64) -> u64 {
    let result: u64;
    if y > x {
        if y.is_multiple_of(2) {
            result = (y * y) - (x - 1);
        } else {
            result = (y - 1) * (y - 1) + 1 + (x - 1);
        }
    } else {
        if x.is_multiple_of(2) {
            result = (x - 1) * (x - 1) + 1 + (y - 1);
        } else {
            result = x * x - (y - 1);
        }
    }
    result
}
