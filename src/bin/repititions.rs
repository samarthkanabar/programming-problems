// https://cses.fi/problemset/task/1069

use std::{cmp::max, io};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let mut max_count = 0u32;
    let mut count = 0u32;
    let mut prev = 'X';

    for ch in input.chars() {
        if ch == prev {
            count += 1;
        } else {
            max_count = max(count, max_count);
            prev = ch;
            count = 1;
        }
    }

    println!("{max_count}");
}
