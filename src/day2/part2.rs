use std::io::{stdin, Read};

enum Direction {
    Forward(u8),
    Down(u8),
    Up(u8)
}

fn process_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');

            let direction = split.next().unwrap();
            let distance = split.next().unwrap().parse().unwrap();

            if direction == "forward" {
                Direction::Forward(distance)
            } else if direction == "down" {
                Direction::Down(distance)
            } else {  // direction == "up"
                Direction::Up(distance)
            }
        })
        .collect()
}

fn follow_trajectory(trajectory: Vec<Direction>) -> (isize, isize) {
    let mut position = (0, 0, 0);

    for direction in trajectory {
        match direction {
            Direction::Forward(distance) => {
                position.0 += distance as isize;
                position.1 += position.2 * distance as isize;
            },
            Direction::Down(distance) => position.2 += distance as isize,
            Direction::Up(distance) => position.2 -= distance as isize,
        }
    }

    (position.0, position.1)
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let data = process_input(&buffer);

    let (horiz, vert) = follow_trajectory(data);

    println!("{}", horiz * vert);
}
