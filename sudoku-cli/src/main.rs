use clap::Parser;
use sudoku_engine::Sudoku;

mod commands;

use commands::{Cli, Command};

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Generate { difficulty, size } => {
            let sudoku = Sudoku::generate(size.into(), difficulty.into());
            print_sudoku(&sudoku.play)
        }
    }
}

pub fn print_sudoku(sudoku: &Vec<Vec<u8>>) {
    let size = sudoku.len();
    let size_sqrt = (size as f64).sqrt() as usize;

    println!(
        "{}",
        (0..(size + size_sqrt))
            .map(|_| "--")
            .collect::<Vec<&str>>()
            .join("")
    );
    for row in 0..size {
        for column in 0..size {
            if (column + 1) % size_sqrt != 0 {
                print!(
                    " {}",
                    if sudoku[row][column] != 0 {
                        sudoku[row][column].to_string()
                    } else {
                        "_".to_string()
                    }
                )
            } else {
                print!(
                    " {} |",
                    if sudoku[row][column] != 0 {
                        sudoku[row][column].to_string()
                    } else {
                        "_".to_string()
                    }
                )
            }
        }
        if (row + 1) % size_sqrt == 0 {
            println!();
            println!(
                "{}",
                (0..(size + size_sqrt))
                    .map(|_| "--")
                    .collect::<Vec<&str>>()
                    .join("")
            );
        } else {
            println!()
        }
    }
}
