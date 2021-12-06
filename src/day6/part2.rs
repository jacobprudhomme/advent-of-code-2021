use std::io::{stdin, Read};

fn process_input(input: &str) -> [u64; 9] {
    let mut init_population = [0; 9];

    input
        .trim()
        .split(',')
        .for_each(|num| {
            init_population[num.parse::<usize>().unwrap()] += 1
        });

    init_population
}

fn grow_population(mut population: [u64; 9], generations: u16) -> u64 {
    for _ in 0..generations {
        let new_fish = population[0];
        for time_left in 0..8 {
            population[time_left] = population[time_left+1];
        }
        population[8] = new_fish;
        population[6] += new_fish;
    }

    let mut sum = 0;
    for num_fish in population {
        sum += num_fish;
    }

    sum
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let init_population = process_input(&buffer);

    let count = grow_population(init_population, 256);

    println!("{}", count);
}
