use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn compare_depths(data: Vec<u16>) -> usize {
    data
        .windows(2)
        .filter(|depths| depths[1] > depths[0])
        .count()
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
