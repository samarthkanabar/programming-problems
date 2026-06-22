// https://cses.fi/problemset/task/1072

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    for i in 1..=n {
        if i == 1 {
            println!("0");
        } else {
            println!("{}", ((i * i) * (i * i - 1)) / 2 - (4 * (i - 1) * (i - 2)));
        }
    }
}
