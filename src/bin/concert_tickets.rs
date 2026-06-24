// https://cses.fi/problemset/task/1091

use std::{
    collections::BTreeMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut tickets: BTreeMap<i64, usize> = BTreeMap::new();

    for _ in 0..n {
        let price: i64 = it.next().unwrap().parse().unwrap();
        *tickets.entry(price).or_insert(0) += 1;
    }

    let mut out = String::new();

    for _ in 0..m {
        let max_price: i64 = it.next().unwrap().parse().unwrap();

        if let Some((&price, _)) = tickets.range(..=max_price).next_back() {
            out.push_str(&format!("{price}\n"));
            let entry = tickets.get_mut(&price).unwrap();
            *entry -= 1;
            if *entry == 0 {
                tickets.remove(&price);
            }
        } else {
            out.push_str("-1\n");
        }
    }

    println!("{out}");
}
