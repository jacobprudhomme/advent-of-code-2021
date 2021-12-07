use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<u16> {
    input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn sum_to_n(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn find_best_fuel_usage(positions: Vec<u16>) -> u32 {
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    let mut best_fuel_usage = u32::MAX;
    for curr_pos in min..=max {
        let fuel_usage = positions
            .iter()
            .map(|&pos| sum_to_n((pos as i16 - curr_pos as i16).abs() as u32))
            .sum::<u32>();

        if fuel_usage < best_fuel_usage {
            best_fuel_usage = fuel_usage;
        }
    }

    best_fuel_usage
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let crab_positions = process_input(&buffer);

    let best_fuel_usage = find_best_fuel_usage(crab_positions);

    println!("{}", best_fuel_usage);
}
