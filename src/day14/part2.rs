use std::io::{stdin, Read};

use indexmap::IndexMap;
use itertools::Itertools;

fn process_input(input: &str) -> (IndexMap<String, u64>, (char, char), IndexMap<String, char>) {
    let mut segments = input.split("\n\n");

    let template = segments
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let first = template[0];
    let last = template.last().copied().unwrap();

    let template = template
        .windows(2)
        .fold(IndexMap::new(), |mut acc, pair| {
            let pair = pair.iter().collect::<String>();
            *acc.entry(pair).or_insert(0) += 1;
            acc
        });

    let rules = segments
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut rule = line.split(" -> ");
            (
                rule.next().unwrap().to_string(),
                rule.next().unwrap().chars().next().unwrap()
            )
        })
        .collect();

    (template, (first, last), rules)
}

fn insert_rules(template: &mut IndexMap<String, u64>, rules: &IndexMap<String, char>) {
    let mut new_pairs = Vec::new();
    for (pair, count) in template.iter_mut() {
        if *count > 0 && rules.contains_key(pair) {
            let new_elem = rules[pair];
            let mut new_segment = pair.clone();
            new_segment.insert(1, new_elem);

            new_pairs.push((new_segment[0..2].to_string(), *count));
            new_pairs.push((new_segment[1..3].to_string(), *count));

            *count = 0;
        }
    }

    for (pair, count) in new_pairs {
        *template.entry(pair).or_insert(0) += count;
    }
}

fn repeat_rules(template: &mut IndexMap<String, u64>, rules: IndexMap<String, char>, n: u8) {
    for _ in 0..n {
        insert_rules(template, &rules);
    }
}

fn count_least_and_most_common_elem(polymer: IndexMap<String, u64>, (first, last): (char, char)) -> (u64, u64) {
    let mut char_frequencies = polymer
        .iter()
        .fold(IndexMap::new(), |mut acc, (pair, count)| {
            let mut chars_in_pair = pair.chars();
            *acc.entry(chars_in_pair.next().unwrap()).or_insert(0) += count;
            *acc.entry(chars_in_pair.next().unwrap()).or_insert(0) += count;
            acc
        });
    // Every character is double-counted, except for those on the ends, so add 1 to their counts
    char_frequencies.entry(first).and_modify(|count| *count += 1);
    char_frequencies.entry(last).and_modify(|count| *count += 1);

    for (_, count) in char_frequencies.iter_mut() {
        *count /= 2;
    }

    let (min, max) = char_frequencies
        .values()
        .minmax()
        .into_option()
        .unwrap();

    (*min, *max)
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let (mut template, bounds, rules) = process_input(&buffer);

    repeat_rules(&mut template, rules, 40);
    let (min, max) = count_least_and_most_common_elem(template, bounds);

    println!("{}", max - min);
}
