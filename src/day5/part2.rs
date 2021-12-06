use std::{io::{stdin, Read}, iter::repeat};

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
        .collect()
}

fn fill_grid(lines: Vec<Line>) -> IndexMap<Point, u8> {
    let mut grid = IndexMap::new();

    for line in lines {
        let x_start = line.0.x;
        let x_end = line.1.x;
        let y_start = line.0.y;
        let y_end = line.1.y;

        if x_start < x_end && y_start < y_end {
            for (x, y) in (x_start..=x_end).zip(y_start..=y_end) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start == x_end && y_start < y_end {
            for (x, y) in (repeat(x_start)).zip(y_start..=y_end) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start > x_end && y_start < y_end {
            for (x, y) in (x_end..=x_start).rev().zip(y_start..=y_end) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start < x_end && y_start == y_end {
            for (x, y) in (x_start..=x_end).zip(repeat(y_start)) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start == x_end && y_start == y_end {
            for (x, y) in (repeat(x_start)).zip(repeat(y_start)) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start > x_end && y_start == y_end {
            for (x, y) in (x_end..=x_start).rev().zip(repeat(y_start)) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start < x_end && y_start > y_end {
            for (x, y) in (x_start..=x_end).zip((y_end..=y_start).rev()) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else if x_start == x_end && y_start > y_end {
            for (x, y) in (repeat(x_start)).zip((y_end..=y_start).rev()) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        } else {
            for (x, y) in (x_end..=x_start).rev().zip((y_end..=y_start).rev()) {
                *grid.entry(Point { x, y }).or_insert(0) += 1;
            }
        }

        // for point in range {
        //     let point = match point {
        //         Both(x, y) => Point { x, y },  // 45-degree diagonal line
        //         Left(x) => Point { x, y: y_start },  // Horizontal line (y coordinate is same for entire line)
        //         Right(y) => Point { x: x_start, y },  // Vertical line (x coordinate is same for entire line)
        //     };
        //     println!("Adding to point {:?}", point);

        //     *grid.entry(point).or_insert(0) += 1;
        // }
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
