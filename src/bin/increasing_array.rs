// https://cses.fi/problemset/task/1094

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let mut prev = 0u64;
    let mut moves = 0u64;
    for item in input.split_whitespace() {
        let num: u64 = item.parse().expect("should only be numbers");

        if num < prev {
            moves = moves + (prev - num);
        } else {
            prev = num;
        }
    }

    println!("{moves}");
}
