use std::io::{stdin, Read};

trait Delim {
    fn is_opening_delim(&self) -> bool;

    fn is_closing_delim_of(&self, other: &char) -> bool;

    fn get_closing_delim(&self) -> char;

    fn get_score(&self) -> u64;
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

    fn get_closing_delim(&self) -> char {
        match self {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => ' ',  // Will never reach this case
        }
    }

    fn get_score(&self) -> u64 {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
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

fn parse_line(line: Vec<char>) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for delim in line {
        if delim.is_opening_delim() {
            stack.push(delim);
        } else if delim.is_closing_delim_of(stack.last().unwrap()) {
            stack.pop();
        } else {
            return None;  // Corrupt line
        }
    }

    if !stack.is_empty() {
        Some(stack)  // Incomplete line
    } else {
        None  // Correct line
    }
}

fn calculate_score(lines: Vec<Vec<char>>) -> u64 {
    let mut scores = Vec::new();
    for line in lines {
        if let Some(delims) = parse_line(line) {
            let score = delims
                .iter()
                .rev()  // Reverse the stack
                .map(|delim| delim.get_closing_delim())
                .fold(0, |acc, delim|  acc * 5 + delim.get_score());

            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
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
