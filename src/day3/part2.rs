use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<Vec<u32>> {
    let mut length = 0;

    let rows: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            length += 1;

            let digits = line.chars();
            digits.map(|digit| digit.to_digit(2).unwrap())
            .collect::<Vec<u32>>()
        })
        .collect();

    rows
}

fn calculate_column_sums(rows: &[Vec<u32>]) -> Vec<u32> {
    rows
        .iter()
        .map(|line| line.to_vec())
        .reduce(|acc, line| {
            let pairs = acc.iter().zip(line.iter());
            pairs.map(|(x, y)| x + y).collect()
        })
        .unwrap()
}

fn determine_most_common_digit(column_sum: u32, threshold: u32) -> u32 {
    if column_sum >= threshold { 1 } else { 0 }
}

fn determine_least_common_digit(column_sum: u32, threshold: u32) -> u32 {
    1 - determine_most_common_digit(column_sum, threshold)
}

fn determine_ratings(rows: Vec<Vec<u32>>) -> (u32, u32) {
    let row_length = rows[0].len();

    let mut possible_o2_gen_ratings = rows.clone();
    let mut possible_co2_scrub_ratings = rows;
    for i in 0..row_length {
        let o2_gen_column_sums = calculate_column_sums(&possible_o2_gen_ratings);
        let co2_scrub_column_sums = calculate_column_sums(&possible_co2_scrub_ratings);

        let most_common_digit = determine_most_common_digit(
            o2_gen_column_sums[i],
            ((possible_o2_gen_ratings.len() + 1) / 2) as u32
        );  // Ceiling integer division
        let least_common_digit = determine_least_common_digit(
            co2_scrub_column_sums[i],
            ((possible_co2_scrub_ratings.len() + 1) / 2) as u32
        );  // Ceiling integer division

        if possible_o2_gen_ratings.len() == 1 && possible_co2_scrub_ratings.len() == 1 { break; }

        if possible_o2_gen_ratings.len() > 1 {
            possible_o2_gen_ratings = possible_o2_gen_ratings
                .into_iter()
                .filter(|row| row[i] == most_common_digit)
                .collect();
        }

        if possible_co2_scrub_ratings.len() > 1 {
            possible_co2_scrub_ratings = possible_co2_scrub_ratings
                .into_iter()
                .filter(|row| row[i] == least_common_digit)
                .collect();
        }
    }

    let mut o2_gen_rating = 0b0;
    let mut co2_scrub_rating = 0b0;
    for i in 0..row_length {
        o2_gen_rating *= 2;
        o2_gen_rating += possible_o2_gen_ratings[0][i];

        co2_scrub_rating *= 2;
        co2_scrub_rating += possible_co2_scrub_ratings[0][i];
    }

    (o2_gen_rating, co2_scrub_rating)
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let columns = process_input(&buffer);

    let (o2_gen_rating, co2_scrub_rating) = determine_ratings(columns);

    println!("{}", o2_gen_rating * co2_scrub_rating);
}
