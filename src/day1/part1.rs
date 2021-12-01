use itertools::Itertools;
use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn compare_depths(data: Vec<u16>) -> u16 {
    data
        .iter()
        .tuple_windows()
        .fold(0, |acc, (prev_depth, curr_depth)| {
            if curr_depth > prev_depth { acc + 1 } else { acc }
        })
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let data = process_input(&buffer);

    let count = compare_depths(data);

    println!("{}", count);
}
