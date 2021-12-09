use std::io::{stdin, Read};

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

fn find_low_points(height_map: Vec<Vec<u8>>) -> Vec<u8> {
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
                low_points.push(height_at_point);
            }
        }
    }

    low_points
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let height_map = process_input(&buffer);

    let low_points = find_low_points(height_map);

    println!("{}", low_points.iter().map(|x| (x+1) as u16).sum::<u16>());
}
