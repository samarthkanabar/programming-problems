//https://cses.fi/problemset/task/1068

use std::io;

fn main() {
    let input: String = {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read from stdin");
        buf
    };

    let mut n: u64 = input.parse().expect("should input a number");

    loop {
        print!("{n}");

        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }

        print!(" ");
    }
}
