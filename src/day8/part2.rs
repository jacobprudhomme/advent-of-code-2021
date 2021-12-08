use std::io::{stdin, Read};

use indexmap::{IndexMap, IndexSet};

struct Entry {
    signals: Vec<String>,
    outputs: Vec<String>,
}

fn process_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut signals_and_outputs = line.split(" | ");
            let signals = signals_and_outputs.next().unwrap().split(' ').map(|signal| signal.to_string()).collect();
            let outputs = signals_and_outputs.next().unwrap().split(' ').map(|signal| signal.to_string()).collect();

            Entry { signals, outputs }
        })
        .collect()
}

fn determine_signals(entry: &Entry) -> IndexMap<String, u8> {
    let mut signals = entry.signals
        .iter()
        .map(|signal| {
            let mut new_signal = signal.chars().collect::<Vec<char>>();
            new_signal.sort_unstable();  // Sort the segments within each signal, to facilitate comparison later
            new_signal.iter().collect()
        })
        .collect::<Vec<String>>();
    signals.sort_by_key(|a| a.len());

    let mut signals_to_digits = IndexMap::new();
    signals_to_digits.insert(signals[0].to_string(), 1);  // 2 segments => 1
    signals_to_digits.insert(signals[1].to_string(), 7);  // 3 segments => 7
    signals_to_digits.insert(signals[2].to_string(), 4);  // 4 segments => 4
    signals_to_digits.insert(signals[9].to_string(), 8);  // 7 segments => 8

    for signal in &signals[3..6] {  // 5 segments => 2, 3 or 5
        let segments = IndexSet::<_>::from_iter(signal.chars());
        if segments.intersection(&IndexSet::<_>::from_iter(signals[0].chars())).collect::<IndexSet<&char>>().len() == 2 {
            signals_to_digits.insert(signal.to_string(), 3);  // Shares 2 segments with 1 => 3
        } else if segments.intersection(&IndexSet::<_>::from_iter(signals[2].chars())).collect::<IndexSet<&char>>().len() == 2 {
            signals_to_digits.insert(signal.to_string(), 2);  // Shares 2 segments with 4 => 2
        } else {
            signals_to_digits.insert(signal.to_string(), 5);  // Otherwise => 5
        }
    }

    for signal in &signals[6..9] {  // 6 segments => 0, 6 or 9
        let segments = IndexSet::<_>::from_iter(signal.chars());
        if segments.intersection(&IndexSet::<_>::from_iter(signals[0].chars())).collect::<IndexSet<&char>>().len() == 1 {
            signals_to_digits.insert(signal.to_string(), 6);  // Shares 1 segment with 1 => 6
        } else if segments.intersection(&IndexSet::<_>::from_iter(signals[2].chars())).collect::<IndexSet<&char>>().len() == 4 {
            signals_to_digits.insert(signal.to_string(), 9);  // Shares 4 segments with 4 => 9
        } else {
            signals_to_digits.insert(signal.to_string(), 0);  // Otherwise => 0
        }
    }

    signals_to_digits
}

fn determine_unique_digit_value(output: &str) -> u8 {
    match output.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => 0,
    }
}

fn determine_output_value(entry: &Entry) -> u16 {
    let mut num = 0;

    // First determine the easy ones (the ones that have a unique solution)
    let mut which_determined = Vec::new();
    for (i, output) in entry.outputs.iter().enumerate() {
        if [2,3,4,7].contains(&output.len()) {
            num += (determine_unique_digit_value(output) as u16) * u16::pow(10, 3 - i as u32);
            which_determined.push(i);
        }
    }

    // Then determine the other ones
    if which_determined.len() < 4 {
        let signals_to_digits = determine_signals(entry);

        let remaining_outputs = entry.outputs
            .iter()
            .enumerate()
            .filter(|(i, _)| !which_determined.contains(i));
        for (i, output) in remaining_outputs {
            let mut output = output.chars().collect::<Vec<char>>();
            output.sort_unstable();
            let output = output.iter().collect::<String>();
            num += (*signals_to_digits.get(&output).unwrap() as u16) * u16::pow(10, 3 - i as u32);
        }
    }

    num
}

fn determine_output_values(entries: Vec<Entry>) -> u32 {
    entries
        .iter()
        .map(|entry| determine_output_value(entry) as u32)
        .sum()
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let entries = process_input(&buffer);

    let unique_output_digits = determine_output_values(entries);

    println!("{}", unique_output_digits);
}
