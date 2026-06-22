// https://cses.fi/problemset/task/1617

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    println!("{}", modpow(n));
}

fn modpow(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let mut u: u64 = modpow(n / 2);
        u = (u * u) % (10_u64.pow(9) + 7);
        if n % 2 == 1 {
            u = (u * 2) % (10_u64.pow(9) + 7);
        }
        u
    }
}
