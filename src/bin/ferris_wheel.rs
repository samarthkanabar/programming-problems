// https://cses.fi/problemset/task/1090

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut line = input.split_whitespace();

    let num_children: usize = line.next().unwrap().parse().unwrap();
    let max_weight: usize = line.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut children_weights: Vec<usize> = Vec::new();
    for num in input.split_whitespace() {
        children_weights.push(num.parse().unwrap());
    }

    children_weights.sort_by(|a, b| b.cmp(a));

    let mut gondola_total_count: usize = 0;
    let mut current_gondola_weight: usize = max_weight;
    let mut start: usize = 0;
    let mut end: usize = num_children - 1;
    let mut gondola_occupancy: usize = 0;

    while start <= end {
        if gondola_occupancy >= 2 {
            gondola_total_count += 1;
            current_gondola_weight = 0;
            gondola_occupancy = 0;
        }
        if current_gondola_weight + *children_weights.get(start).unwrap() <= max_weight {
            current_gondola_weight += *children_weights.get(start).unwrap();
            gondola_occupancy += 1;
            start += 1;
        } else if current_gondola_weight + *children_weights.get(end).unwrap() <= max_weight {
            current_gondola_weight += *children_weights.get(end).unwrap();
            gondola_occupancy += 1;
            end -= 1;
        } else {
            gondola_total_count += 1;
            current_gondola_weight = 0;
            gondola_occupancy = 0;
        }
    }

    println!("{gondola_total_count}");
}
