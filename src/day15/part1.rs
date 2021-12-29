use std::{io::{stdin, Read}, collections::BinaryHeap, cmp::Reverse};

use indexmap::indexmap;

type Point = (u8,u8);

fn process_input(input: &str) -> (Vec<Vec<u8>>, Point) {
    let grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|risk| risk.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let end = ((grid[0].len() - 1) as u8, (grid.len() - 1) as u8);

    (grid, end)
}

fn within_bounds(x: i8, y: i8, end_x: i8, end_y: i8) -> bool {
    0 <= x && x <= end_x && 0 <= y && y <= end_y
}

fn find_neighbouring_coords(coord: Point, end: Point) -> Vec<Point> {
    let rel_neighbours = [(-1,0), (0,-1), (0,1), (1,0)];

    rel_neighbours
        .into_iter()
        .map(|(dx,dy)| (dx as i8, dy as i8))
        .map(|(dx,dy)| (coord.0 as i8 + dx, coord.1 as i8 + dy))
        .filter(|(x,y)| within_bounds(*x, *y, end.0 as i8, end.1 as i8))
        .map(|(x,y)| (x as u8, y as u8))
        .collect()
}

fn find_least_risky_path(grid: Vec<Vec<u8>>, end: Point) -> u16 {
    let start = (0,0);

    let mut to_visit = BinaryHeap::from([(Reverse(0), start)]);
    let mut path_costs = indexmap!{ start => 0 };

    while let Some((cost, coord)) = to_visit.pop() {
        if coord == end { return path_costs[&coord]; }

        let Reverse(cost) = cost;

        if cost > *path_costs.get(&coord).unwrap_or(&u16::MAX) { continue; }

        for neighbour @ (x,y) in find_neighbouring_coords(coord, end) {
            let cost_to_here = cost + grid[y as usize][x as usize] as u16;

            if cost_to_here < *path_costs.get(&neighbour).unwrap_or(&u16::MAX) {
                to_visit.push((Reverse(cost_to_here), (x,y)));
                *path_costs.entry(neighbour).or_insert(0) = cost_to_here;
            }
        }
    }

    0  // Should never get here
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let (grid, end) = process_input(&buffer);

    let path = find_least_risky_path(grid, end);

    println!("{:?}", path);
}
