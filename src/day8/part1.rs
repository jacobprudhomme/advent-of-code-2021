use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .flat_map(|line| {
            let mut signals_and_outputs = line.split(" | ");
            signals_and_outputs.next();  // Ignore signals
            signals_and_outputs
                .next()
                .unwrap()
                .split(' ')
        })
        .collect()
}

fn count_unique_output_digits(output_digits: Vec<&str>) -> usize {
    output_digits
        .iter()
        .map(|output_digit| output_digit.len())
        .filter(|num_segments| [2,3,4,7].contains(num_segments))  // output is "1", "7", "4" or "8"
        .count()
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let output_digits = process_input(&buffer);

    let unique_output_digits = count_unique_output_digits(output_digits);

    println!("{}", unique_output_digits);
}
