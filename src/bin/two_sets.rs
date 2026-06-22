// https://cses.fi/problemset/task/1092

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    let sum: i64 = n * (n + 1) / 2;

    if sum % 2 == 1 {
        println!("NO");
    } else {
        let mut target: i64 = sum / 2;
        let mut counter: i64 = n;
        let mut list1: Vec<i64> = Vec::new();
        let mut list2: Vec<i64> = Vec::new();
        while counter > 0 {
            if target - counter >= 0 {
                target -= counter;
                list1.push(counter);
            } else {
                list2.push(counter);
            }
            counter -= 1;
        }
        if target != 0 {
            println!("NO");
        } else {
            println!("YES");
            println!("{}", list1.len());
            for num in list1 {
                print!("{num} ");
            }
            println!();
            println!("{}", list2.len());
            for num in list2 {
                print!("{num} ");
            }
            println!();
        }
    }
}
