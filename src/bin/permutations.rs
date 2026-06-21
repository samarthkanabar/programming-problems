// https://cses.fi/problemset/task/1070

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let n: u32 = input.trim().parse().expect("should input a number");

    if n == 1 {
        println!("1");
    } else if n < 4 {
        println!("NO SOLUTION");
    } else {
        let mut counter = 2u32;

        while counter <= n {
            print!("{counter} ");
            counter += 2;
        }

        counter = 1u32;

        while counter <= n {
            print!("{counter} ");
            counter += 2;
        }
    }
}
