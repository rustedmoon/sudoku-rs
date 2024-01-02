use std::collections::HashSet;

// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Idiot,
    Beginner,
    Easy,
    Medium,
    Hard,
    Impossible,
    Mortal,
    God,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardSize {
    Four = 4,
    Nine = 9,
    Sixteen = 16,
}

pub struct Sudoku {
    pub difficulty: Difficulty,
    pub template: Vec<Vec<u8>>,
    pub play: Vec<Vec<u8>>,
}

impl Sudoku {
    /// Generate a sudoku with specified quadrant edge size and remove number based on chosen difficulty.
    pub fn generate(size: BoardSize, difficulty: Difficulty) -> Self {
        // Generate sudoku
        let mut sudoku = generate_empty(size);
        solve_sudoku(&mut sudoku);

        Self {
            difficulty,
            template: sudoku.clone(),
            play: sudoku,
        }
    }

    pub fn verify(&self) -> bool {
        verify(&self.play)
    }

    pub fn add_number(&mut self, num: u8, row: usize, column: usize) {
        todo!()
    }
}

fn generate_empty(size: BoardSize) -> Vec<Vec<u8>> {
    vec![vec![0u8; size as usize]; size as usize]
}

fn is_valid(sudoku: &Vec<Vec<u8>>, size_sqrt: usize, row: usize, column: usize, num: u8) -> bool {
    let size = sudoku.len();
    for i in 0..(size) {
        if (sudoku[row][i] == num && i != column)
            || (sudoku[i][column] == num && i != row)
            || (sudoku[row - row % size_sqrt + (i + size_sqrt) / size_sqrt - 1]
                [column - column % size_sqrt + (i) % size_sqrt]
                == num
                && sudoku[row - row % size_sqrt + (i + size_sqrt) / size_sqrt - 1]
                    [column - column % size_sqrt + (i) % size_sqrt]
                    != sudoku[row][column])
        {
            return false;
        }
    }
    true
}

fn _solve(sudoku: &mut Vec<Vec<u8>>, size: usize, size_sqrt: usize, numbers: &HashSet<u8>) -> bool {
    for row in 0..size {
        let row_number: HashSet<u8> = sudoku[row].iter().copied().collect();
        for column in 0..size {
            if sudoku[row][column] == 0 {
                for num in numbers.difference(&row_number) {
                    if is_valid(sudoku, size_sqrt, row, column, *num) {
                        sudoku[row][column] = *num;
                        if _solve(sudoku, size, size_sqrt, numbers) {
                            return true;
                        }
                        sudoku[row][column] = 0
                    }
                }
                return false;
            }
        }
    }
    true
}

fn solve_sudoku(sudoku: &mut Vec<Vec<u8>>) -> bool {
    let size = sudoku.len();
    let size_sqrt = (size as f64).sqrt() as usize;
    let numbers: HashSet<u8> = (1..=size as u8).collect();
    _solve(sudoku, size, size_sqrt, &numbers)
}

fn verify(sudoku: &Vec<Vec<u8>>) -> bool {
    let size = sudoku.len();
    let size_sqrt = (size as f64).sqrt() as usize;

    for row in 0..size {
        for column in 0..size {
            if !is_valid(sudoku, size_sqrt, row, column, sudoku[row][column]) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku() {
        assert!(verify(&vec![
            vec![1, 2, 6, 4, 3, 7, 5, 9, 8],
            vec![9, 4, 3, 6, 5, 8, 1, 2, 7],
            vec![7, 8, 5, 2, 1, 9, 3, 4, 6],
            vec![8, 6, 7, 3, 9, 2, 4, 5, 1],
            vec![3, 9, 4, 5, 7, 1, 8, 6, 2],
            vec![5, 1, 2, 8, 4, 6, 7, 3, 9],
            vec![6, 5, 1, 7, 2, 4, 9, 8, 3],
            vec![2, 3, 9, 1, 8, 5, 6, 7, 4],
            vec![4, 7, 8, 9, 6, 3, 2, 1, 5],
        ]));
    }

    #[test]
    fn test_invalid_sudoku() {
        assert!(!verify(&vec![
            vec![1, 2, 6, 4, 3, 7, 5, 9, 8],
            vec![9, 4, 3, 6, 5, 8, 1, 2, 7],
            vec![7, 8, 5, 2, 1, 9, 3, 4, 6],
            vec![8, 6, 7, 3, 9, 2, 4, 5, 1],
            vec![3, 9, 4, 5, 7, 1, 8, 6, 2],
            vec![5, 1, 2, 8, 4, 6, 7, 3, 9],
            vec![6, 5, 1, 7, 2, 4, 9, 8, 3],
            vec![2, 3, 9, 1, 8, 5, 6, 7, 4],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 4],
        ]));
        assert!(!verify(&vec![
            vec![1, 2, 6, 4, 3, 7, 5, 9, 8],
            vec![9, 4, 3, 6, 5, 8, 1, 2, 7],
            vec![7, 8, 5, 2, 1, 9, 3, 4, 6],
            vec![8, 6, 7, 3, 9, 2, 4, 5, 1],
            vec![3, 9, 4, 5, 7, 1, 8, 6, 2],
            vec![5, 1, 2, 8, 4, 6, 7, 3, 9],
            vec![6, 5, 1, 7, 2, 4, 9, 8, 3],
            vec![2, 3, 9, 1, 8, 5, 6, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 1, 2, 3],
        ]));
        assert!(!verify(&vec![
            vec![1, 2, 6, 4, 3, 7, 5, 9, 8],
            vec![9, 4, 3, 6, 5, 8, 1, 2, 7],
            vec![7, 8, 5, 2, 1, 9, 3, 4, 6],
            vec![8, 6, 7, 3, 9, 2, 4, 5, 1],
            vec![3, 9, 4, 5, 7, 1, 8, 6, 2],
            vec![5, 1, 2, 8, 4, 6, 7, 3, 9],
            vec![6, 5, 1, 7, 2, 4, 9, 8, 3],
            vec![2, 3, 9, 1, 8, 5, 6, 7, 4],
            vec![4, 7, 8, 9, 6, 1, 2, 1, 5],
        ]));
    }

    #[test]
    fn test_4x4_sudoku() {
        let sudoku = Sudoku::generate(BoardSize::Four, Difficulty::Beginner);
        assert!(verify(&sudoku.template));
    }

    #[test]
    fn test_9x9_sudoku() {
        let sudoku = Sudoku::generate(BoardSize::Nine, Difficulty::Beginner);
        assert!(verify(&sudoku.template));
    }

    #[test]
    fn test_16x16_sudoku() {
        let sudoku = Sudoku::generate(BoardSize::Sixteen, Difficulty::Beginner);
        assert!(verify(&sudoku.template));
    }
}
