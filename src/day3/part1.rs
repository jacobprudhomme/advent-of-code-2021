use std::io::{stdin, Read};

fn process_input(input: &str) -> (Vec<u32>, u32) {
    let mut length = 0;

    let column_sums = input
        .lines()
        .map(|line| {
            length += 1;

            let digits = line.chars();
            digits
                .map(|digit| digit.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .reduce(|acc, line| {
            let pairs = acc.iter().zip(line.iter());
            pairs
                .map(|(x, y)| x + y)
                .collect()
        })
        .unwrap();

    (column_sums, length / 2)
}

fn determine_rates((column_sums, threshold): (Vec<u32>, u32)) -> (u32, u32) {
    let mut gamma_rate = 0b0;
    let mut epsilon_rate = 0b0;

    for sum in column_sums {
        if sum > threshold {
            gamma_rate *= 2;
            gamma_rate += 1;

            epsilon_rate *= 2;
        } else {
            gamma_rate *= 2;

            epsilon_rate *= 2;
            epsilon_rate += 1;
        }
    }

    (gamma_rate, epsilon_rate)
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let data = process_input(&buffer);

    let (gamma_rate, epsilon_rate) = determine_rates(data);

    println!("{}", gamma_rate * epsilon_rate);
}
