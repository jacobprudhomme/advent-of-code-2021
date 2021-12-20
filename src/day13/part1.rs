use std::io::{stdin, Read};

use indexmap::IndexSet;

type Point = (u16, u16);

#[derive(Clone, Copy)]
enum Fold {
    Horiz(u16),
    Vert(u16),
}

fn process_input(input: &str) -> (IndexSet<Point>, Vec<Fold>) {
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

    (points, folds)
}

fn do_fold(mut points: IndexSet<Point>, fold: Fold) -> IndexSet<Point> {
    match fold {
        Fold::Horiz(pos) => {
            for coord in points.clone().iter().filter(|(_, y_coord)| *y_coord >= pos) {
                points.remove(coord);
                points.insert((coord.0, 2 * pos - coord.1));
            }
        },
        Fold::Vert(pos) => {
            for coord in points.clone().iter().filter(|(x_coord, _)| *x_coord >= pos) {
                points.remove(coord);
                points.insert((2 * pos - coord.0, coord.1));
            }
        },
    }

    points
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let (points, folds) = process_input(&buffer);

    let num_points = do_fold(points, folds[0]).len();

    println!("{}", num_points);
}
