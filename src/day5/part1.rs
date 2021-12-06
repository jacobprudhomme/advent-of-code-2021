use std::io::{stdin, Read};

use indexmap::IndexMap;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

struct Line(Point, Point);

fn process_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut endpoints = line.split(" -> ");
            let (x1, y1) = endpoints
                .next()
                .unwrap()
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect_tuple()
                .unwrap();
            let (x2, y2) = endpoints
                .next()
                .unwrap()
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let start = Point { x: x1, y: y1 };
            let end = Point { x: x2, y: y2 };

            Line(start, end)
        })
        .filter(|line| line.0.x == line.1.x || line.0.y == line.1.y)
        .collect()
}

fn fill_grid(lines: Vec<Line>) -> IndexMap<Point, u8> {
    let mut grid = IndexMap::new();

    for line in lines {
        let x_start = if line.0.x > line.1.x { line.1.x } else { line.0.x };
        let x_end = if line.0.x > line.1.x { line.0.x } else { line.1.x };
        let y_start = if line.0.y > line.1.y { line.1.y } else { line.0.y };
        let y_end = if line.0.y > line.1.y { line.0.y } else { line.1.y };

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        }
    }

    grid
}

fn count_overlaps(grid: IndexMap<Point, u8>) -> usize {
    grid
        .values()
        .filter(|&&overlaps| overlaps >= 2)
        .count()
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let lines = process_input(&buffer);

    let num_overlaps = count_overlaps(fill_grid(lines));

    println!("{}", num_overlaps);
}
