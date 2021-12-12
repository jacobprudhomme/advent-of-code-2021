use std::io::{stdin, Read};

use indexmap::IndexSet;

fn process_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn within_bounds(x: isize, y: isize, width: isize, height: isize) -> bool {
    0 <= x && x < width && 0 <= y && y < height
}

fn flash(grid: &mut Vec<Vec<u8>>, x: usize, y: usize, has_flashed: &mut IndexSet<(usize, usize)>) {
    let neighbours: [(i8,i8); 8] = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    let width = grid[0].len();
    let height = grid.len();

    let neighbours = neighbours
        .iter()
        .map(|(dx,dy)| (x as isize + *dx as isize, y as isize + *dy as isize))
        .filter(|(x,y)| within_bounds(*x, *y, width as isize, height as isize))
        .map(|(x,y)| (x as usize, y as usize));

    for (x,y) in neighbours {
        grid[y][x] += 1;

        if grid[y][x] > 9 && !has_flashed.contains(&(x,y)) {
            has_flashed.insert((x,y));
            flash(grid, x, y, has_flashed);
        }
    }
}

fn do_step(grid: &mut Vec<Vec<u8>>) -> bool {
    for x in 0..10 {
        for y in 0..10 {
            grid[y][x] += 1;
        }
    }

    let mut has_flashed = IndexSet::new();
    for x in 0..10 {
        for y in 0..10 {
            if grid[y][x] > 9 && !has_flashed.contains(&(x,y)) {
                has_flashed.insert((x,y));
                flash(grid, x, y, &mut has_flashed);
            }
        }
    }

    for x in 0..10 {
        for y in 0..10 {
            if grid[y][x] > 9 { grid[y][x] = 0; }
        }
    }

    0 == grid
        .iter()
        .map(|row| {
            row.iter().map(|point| *point as u16).sum::<u16>()
        })
        .sum::<u16>()
}

fn do_steps(mut grid: Vec<Vec<u8>>) -> u8 {
    let mut step = 1;
    while !do_step(&mut grid) {
        step += 1;
    }

    step
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let grid = process_input(&buffer);

    let step = do_steps(grid);

    println!("{}", step);
}
