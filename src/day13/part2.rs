use std::io::{stdin, Read};

use indexmap::IndexSet;

type Point = (u16, u16);

enum Fold {
    Horiz(u16),
    Vert(u16),
}

fn process_input(input: &str) -> (IndexSet<Point>, (u16, u16), Vec<Fold>) {
    let mut segments = input.split("\n\n");

    let points = segments
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut coords = line
                .split(',')
                .map(|coord| coord.parse().unwrap());

            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<IndexSet<Point>>();

    let folds = segments
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let fold = &line[11..];
            let mut fold = fold.split('=');

            match fold.next().unwrap() {
                "x" => Fold::Vert(fold.next().unwrap().parse().unwrap()),
                _ => Fold::Horiz(fold.next().unwrap().parse().unwrap()),  // "y"
            }
        })
        .collect();

    let dims = (
        *points.iter().map(|(coord, _)| coord).max().unwrap() + 1,
        *points.iter().map(|(_, coord)| coord).max().unwrap() + 1,
    );

    (points, dims, folds)
}

fn do_fold(points: &mut IndexSet<Point>, (width, height): (u16, u16), fold: Fold) -> (u16, u16) {
    match fold {
        Fold::Horiz(pos) => {
            for coord in points.clone().iter().filter(|(_, y_coord)| *y_coord >= pos) {
                points.remove(coord);
                points.insert((coord.0, 2 * pos - coord.1));
            }

            (width, pos)
        },
        Fold::Vert(pos) => {
            for coord in points.clone().iter().filter(|(x_coord, _)| *x_coord >= pos) {
                points.remove(coord);
                points.insert((2 * pos - coord.0, coord.1));
            }

            (pos, height)
        },
    }
}

fn do_folds(points: &mut IndexSet<Point>, dims: &mut (u16, u16), folds: Vec<Fold>) {
    for fold in folds {
        *dims = do_fold(points, *dims, fold);
    }
}

fn print_points(points: IndexSet<Point>, dims: (u16, u16)) {
    for y in 0..dims.1 {
        for x in 0..dims.0 {
            print!("{}", if points.contains(&(x,y)) { '#' } else { '.' });
        }
        println!();
    }
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let (mut points, mut dims, folds) = process_input(&buffer);

    do_folds(&mut points, &mut dims, folds);

    print_points(points, dims);
}
