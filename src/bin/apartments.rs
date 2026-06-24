// https://cses.fi/problemset/task/1084

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut line = input.split_whitespace();

    let num_applicants: usize = line.next().unwrap().parse().unwrap();
    let num_apartments: usize = line.next().unwrap().parse().unwrap();
    let tolerance: u32 = line.next().unwrap().parse().unwrap();

    input.clear();
    let mut applicant_prefs: Vec<u32> = Vec::new();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        applicant_prefs.push(num.parse().unwrap());
    }

    input.clear();
    let mut apartment_sizes: Vec<u32> = Vec::new();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        apartment_sizes.push(num.parse().unwrap());
    }

    applicant_prefs.sort();
    apartment_sizes.sort();

    let mut applicant_index: usize = 0;
    let mut apartment_index: usize = 0;
    let mut applicants_assigned: usize = 0;

    while applicant_index < num_applicants {
        if apartment_index < num_apartments {
            let applicant_pref = applicant_prefs.get(applicant_index).unwrap();
            let apartment_size = apartment_sizes.get(apartment_index).unwrap();

            if applicant_pref.abs_diff(*apartment_size) <= tolerance {
                applicants_assigned += 1;
                applicant_index += 1;
                apartment_index += 1;
            } else if *apartment_size > *applicant_pref {
                applicant_index += 1;
            } else {
                apartment_index += 1;
            }
        } else {
            break;
        }
    }

    println!("{applicants_assigned}");
}
