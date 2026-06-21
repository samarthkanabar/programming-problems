// https://cses.fi/problemset/task/1083

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let n: u64 = input.trim().parse().expect("should input a number");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let missing = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().expect("should all be numbers"))
        .fold(n * (n + 1) / 2, |acc, x| acc - x);

    println!("{missing}");
}
