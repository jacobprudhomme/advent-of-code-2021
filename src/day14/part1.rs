use std::io::{stdin, Read};

use indexmap::IndexMap;
use itertools::Itertools;

fn process_input(input: &str) -> (Vec<char>, IndexMap<String, char>) {
    let mut segments = input.split("\n\n");

    let template = segments
        .next()
        .unwrap()
        .chars()
        .collect();

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

    (template, rules)
}

fn insert_rules(template: &mut Vec<char>, rules: &IndexMap<String, char>) {
    let mut new_elems = 0;
    for (i, adj_elems) in template.clone().windows(2).enumerate() {
        let adj_elems = adj_elems.iter().collect::<String>();
        if rules.contains_key(&adj_elems) {
            template.insert(i + new_elems + 1, rules[&adj_elems]);
            new_elems += 1;
        }
    }
}

fn repeat_rules(template: &mut Vec<char>, rules: IndexMap<String, char>, n: u8) {
    for _ in 0..n {
        insert_rules(template, &rules);
    }
}

fn count_least_and_most_common_elem(polymer: Vec<char>) -> (u16, u16) {
    let char_frequencies = polymer
        .iter()
        .fold(IndexMap::new(), |mut acc, char| {
            *acc.entry(char).or_insert(0) += 1;
            acc
        });

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
    let (mut template, rules) = process_input(&buffer);

    repeat_rules(&mut template, rules, 10);
    let (min, max) = count_least_and_most_common_elem(template);

    println!("{}", max - min);
}
