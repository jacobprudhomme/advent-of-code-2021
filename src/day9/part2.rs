use std::io::{stdin, Read};

use indexmap::IndexSet;
use itertools::Itertools;

fn process_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|num| num.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_low_points(height_map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let height = height_map.len();
    let width = height_map[0].len();

    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let adjacent_rows = [y.checked_sub(1), if y+1 < height { Some(y+1) } else { None }];
            let adjacent_cols = [x.checked_sub(1), if x+1 < width { Some(x+1) } else { None }];

            let adjacent_rows = adjacent_rows
                .iter()
                .filter_map(|&row| row)
                .cartesian_product([x])
                .map(|(y,x)| (x,y));
            let adjacent_cols = adjacent_cols
                .iter()
                .filter_map(|&col| col)
                .cartesian_product([y]);

            let height_at_point = height_map[y][x];
            if adjacent_cols.chain(adjacent_rows).all(|(x,y)| height_map[y][x] > height_at_point) {
                low_points.push((x,y));
            }
        }
    }

    low_points
}

fn find_adjacent_low_points(height_map: &[Vec<u8>], (x,y): (usize, usize), basin_so_far: &mut IndexSet<(usize, usize)>) {
    let height = height_map.len();
    let width = height_map[0].len();

    let adjacent_rows = [y.checked_sub(1), if y+1 < height { Some(y+1) } else { None }];
    let adjacent_cols = [x.checked_sub(1), if x+1 < width { Some(x+1) } else { None }];

    let adjacent_rows = adjacent_rows
        .iter()
        .filter_map(|&row| row)
        .cartesian_product([x])
        .map(|(y,x)| (x,y));
    let adjacent_cols = adjacent_cols
        .iter()
        .filter_map(|&col| col)
        .cartesian_product([y]);
    let surrounding_points = IndexSet::<_>::from_iter(adjacent_cols.chain(adjacent_rows));

    for (x,y) in surrounding_points.difference(&basin_so_far.clone()) {
        if height_map[*y][*x] < 9 {
            basin_so_far.insert((*x,*y));
            find_adjacent_low_points(height_map, (*x,*y), basin_so_far)
        }
    }
}

fn map_basins(height_map: &[Vec<u8>], low_points: Vec<(usize, usize)>) -> Vec<IndexSet<(usize, usize)>> {
    let mut basins = Vec::new();

    for low_point in low_points {
        let mut basin_so_far = IndexSet::new();
        find_adjacent_low_points(height_map, low_point, &mut basin_so_far);
        basins.push(basin_so_far);
    }

    basins
}

fn calculate_largest_basin_sizes(mut basins: Vec<IndexSet<(usize, usize)>>) -> Vec<usize> {
    basins.sort_by_key(|basin| -(basin.len() as isize));
    basins[0..3]
        .iter()
        .map(|basin| basin.len())
        .collect()
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let height_map = process_input(&buffer);

    let low_points = find_low_points(&height_map);
    let basins = map_basins(&height_map, low_points);
    let largest_basin_sizes = calculate_largest_basin_sizes(basins);

    println!("{}", largest_basin_sizes.iter().product::<usize>());
}
