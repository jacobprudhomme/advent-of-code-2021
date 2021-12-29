use std::{io::{stdin, Read}, collections::BinaryHeap, cmp::Reverse};

use indexmap::indexmap;

type Point = (u16,u16);

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

    let end = ((5 * grid[0].len() - 1) as u16, (5 * grid.len() - 1) as u16);

    (grid, end)
}

fn within_bounds(x: i16, y: i16, end_x: i16, end_y: i16) -> bool {
    0 <= x && x <= end_x && 0 <= y && y <= end_y
}

fn find_neighbouring_coords(coord: Point, end: Point) -> Vec<Point> {
    let rel_neighbours = [(-1,0), (0,-1), (0,1), (1,0)];

    rel_neighbours
        .into_iter()
        .map(|(dx,dy)| (dx as i16, dy as i16))
        .map(|(dx,dy)| (coord.0 as i16 + dx, coord.1 as i16 + dy))
        .filter(|(x,y)| within_bounds(*x, *y, end.0 as i16, end.1 as i16))
        .map(|(x,y)| (x as u16, y as u16))
        .collect()
}

fn calculate_cost_at_point(grid: &[Vec<u8>], (x,y): Point, tile_width: u16, tile_height: u16) -> u16 {
    let x_in_original_tile = (x % tile_width) as usize;
    let y_in_original_tile = (y % tile_height) as usize;
    let cost_in_original_tile = grid[y_in_original_tile][x_in_original_tile] as u16;

    let x_offset = x / tile_width;
    let y_offset = y / tile_height;

    (cost_in_original_tile + x_offset + y_offset - 1) % 9 + 1
}

fn find_least_risky_path(grid: Vec<Vec<u8>>, end: Point) -> u16 {
    let tile_width = grid[0].len();
    let tile_height = grid.len();

    let start = (0,0);

    let mut to_visit = BinaryHeap::from([(Reverse(0), start)]);
    let mut path_costs = indexmap!{ start => 0 };

    while let Some((cost, coord)) = to_visit.pop() {
        if coord == end { return path_costs[&coord]; }

        let Reverse(cost) = cost;

        if cost > *path_costs.get(&coord).unwrap_or(&u16::MAX) { continue; }

        for neighbour @ (x,y) in find_neighbouring_coords(coord, end) {
            let cost_to_here = cost + calculate_cost_at_point(&grid, neighbour, tile_width as u16, tile_height as u16);

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
