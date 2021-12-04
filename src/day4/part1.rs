use std::io::{stdin, Read};

type Board = Vec<Vec<Option<u8>>>;

fn process_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut sections = input.split("\n\n");

    let draws = sections
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let boards = sections
        .map(|board| {
            board
                .lines()
                .map(|row| {
                    row
                        .split_whitespace()
                        .map(|square| Some(square.trim().parse().unwrap()))
                        .collect()
                })
                .collect()
        })
        .collect();

    (draws, boards)
}

fn find_square_on_board(draw: u8, board: &mut Board, (row_counts, col_counts): &mut (Vec<u8>, Vec<u8>)) -> bool {
    let row_size = board[0].len();
    let col_size = board.len();

    for x in 0..row_size {
        for y in 0..col_size {
            if let Some(square) = board[y][x] {
                if square == draw {
                    row_counts[y] += 1;
                    col_counts[x] += 1;

                    board[y][x] = None;

                    return (row_counts[y] as usize) == row_size || (col_counts[x] as usize) == col_size;
                }
            }
        }
    }

    false
}

fn sum_unmarked(board: &Board) -> u16 {
    board
        .iter()
        .flatten()
        .filter(|square| square.is_some())
        .map(|square| square.unwrap() as u16)
        .sum()
}

fn find_square_on_boards(draw: u8, boards: &mut Vec<Board>, counts: &mut Vec<(Vec<u8>, Vec<u8>)>) -> Option<u16> {
    for (i, board) in boards.iter_mut().enumerate() {
        if find_square_on_board(draw, board, &mut counts[i]) {
            return Some(sum_unmarked(board));
        }
    }

    None
}

fn draw_number(draws: Vec<u8>, boards: &mut Vec<Board>) -> u32 {
    let row_size = boards[0][0].len();
    let col_size = boards[0].len();
    let num_boards = boards.len();

    let mut counts = vec![(vec![0; row_size], vec![0; col_size]); num_boards];
    for draw in draws {
        if let Some(sum) = find_square_on_boards(draw, boards, &mut counts) {
            return (sum as u32) * (draw as u32);
        }
    }

    0  // Should never get here
}

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Cannot read input");
    let (draws, mut boards) = process_input(&buffer);

    let score = draw_number(draws, &mut boards);

    println!("{}", score);
}
