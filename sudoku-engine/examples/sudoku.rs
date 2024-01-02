use sudoku_engine::{BoardSize, Difficulty, Sudoku};

fn main() {
    let sudoku = Sudoku::generate(BoardSize::Nine, Difficulty::Impossible);
}
