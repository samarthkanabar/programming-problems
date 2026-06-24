// https://cses.fi/problemset/task/1621

use std::{collections::HashSet, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    if n < 2 {
        println!("{n}");
        return;
    }

    let mut num_set: HashSet<u32> = HashSet::new();
    for num in input.split_whitespace() {
        let num: u32 = num.parse().unwrap();
        num_set.insert(num);
    }

    println!("{}", num_set.len());
}
