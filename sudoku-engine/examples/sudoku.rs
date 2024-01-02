use sudoku_engine::{Difficulty, Sudoku, sudoku_with_diagonal, print_sudoku};

fn main() {
    let sudoku_9 = Sudoku::generate(9, Difficulty::Impossible);
    print_sudoku(&sudoku_9.template);

    let sudoku_4 = Sudoku::generate(4, Difficulty::Impossible);
    print_sudoku(&sudoku_4.template);

    let sudoku_16 = Sudoku::generate(16, Difficulty::Impossible);
    print_sudoku(&sudoku_16.template);
}
