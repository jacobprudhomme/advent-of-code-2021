use std::io::{stdin, Read};

fn process_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn grow_population(mut population: Vec<u8>, generations: u8) -> usize {
    let mut count = population.len();
    for _ in 0..generations {
        let mut new_count = count;
        for i in 0..count {
            if population[i] > 0 {
                population[i] -= 1;
            } else {
                population[i] = 6;
                population.push(8);
                new_count += 1;
            }
        }

        count = new_count;
    }

    count
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let init_population = process_input(&buffer);

    let count = grow_population(init_population, 80);

    println!("{}", count);
}
