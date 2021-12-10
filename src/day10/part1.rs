use std::io::{stdin, Read};

trait Delim {
    fn is_opening_delim(&self) -> bool;

    fn is_closing_delim_of(&self, other: &char) -> bool;

    fn get_score(&self) -> u32;
}

impl Delim for char {
    fn is_opening_delim(&self) -> bool {
        matches!(self, '(' | '[' | '{' | '<')
    }

    fn is_closing_delim_of(&self, other: &char) -> bool {
        matches!((other, self),
            ('(', ')') |
            ('[', ']') |
            ('{', '}') |
            ('<', '>') )
    }

    fn get_score(&self) -> u32 {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,  // Will never reach this case
        }
    }
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_line(line: Vec<char>) -> Option<char> {
    let mut stack = Vec::new();
    for delim in line {
        if delim.is_opening_delim() {
            stack.push(delim);
        } else if delim.is_closing_delim_of(stack.last().unwrap()) {
            stack.pop();
        } else {
            return Some(delim);  // Corrupt line
        }
    }

    None  // Correct or incomplete line
}

fn calculate_score(lines: Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for line in lines {
        if let Some(delim) = parse_line(line) {
            score += delim.get_score();
        }
    }

    score
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let lines = process_input(&buffer);

    let score = calculate_score(lines);

    println!("{}", score);
}
