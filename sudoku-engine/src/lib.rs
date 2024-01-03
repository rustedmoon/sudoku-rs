use std::collections::HashSet;

pub enum Error {
    InvalidNumber,
    InvalidCoordinate,
    StartingNumbers
}

// The number indicate the percentage of total number to be removed, the difficulty scale up with size
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Beginner = 15,
    Easy = 30,
    Medium = 45,
    Hard = 57,
    Extreme = 66,
    Impossible = 79,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardSize {
    Four = 4,
    Nine = 9,
    Sixteen = 16,
}

pub struct Sudoku {
    pub difficulty: Difficulty,
    starting: HashSet<(usize, usize)>,
    pub play: Vec<Vec<u8>>,
}

impl Sudoku {
    /// Generate a sudoku with specified quadrant edge size and remove number based on chosen difficulty.
    pub fn generate(size: BoardSize, difficulty: Difficulty) -> Self {
        // Generate sudoku
        let mut sudoku = generate(size);
        apply_difficulty(&mut sudoku, difficulty);

        // Save starting position that shouldn't be modified by the player
        let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
        for row in 0..size as usize {
            for column in 0..size as usize {
                if sudoku[row][column] != 0 {
                    coordinates.insert((row, column));
                }
            }
        }

        Self {
            difficulty,
            starting: coordinates,
            play: sudoku,
        }
    }

    pub fn verify(&self) -> bool {
        verify(&self.play)
    }

    pub fn add_number(&mut self, num: u8, row: usize, column: usize) -> Result<(), Error> {
        if num as usize > self.play.len() {
            Err(Error::InvalidNumber)
        } else if row > self.play.len() || column > self.play.len() {
            Err(Error::InvalidCoordinate)
        } else if self.starting.contains(&(row, column)) {
            Err(Error::StartingNumbers)
        } else {
            self.play[row - 1][column - 1] = num;
            Ok(())
        }
        
    }

    pub fn remove_number(&mut self, row: usize, column: usize) -> Result<(), Error> {
        if row > self.play.len() || column > self.play.len() {
            Err(Error::InvalidCoordinate)
        } else if self.starting.contains(&(row, column)) {
            Err(Error::StartingNumbers)
        } else {
            self.play[row - 1][column - 1] = 0;
            Ok(())
        }
    }

    
}

fn generate(size: BoardSize) -> Vec<Vec<u8>> {
    let mut sudoku = vec![vec![0u8; size as usize]; size as usize];
    solve_sudoku(&mut sudoku);
    sudoku
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

fn apply_difficulty(sudoku: &mut Vec<Vec<u8>>, difficulty: Difficulty) {
    let size = sudoku.len();
    let difficulty: usize = size * size * difficulty as usize / 100;
    let mut coordinates: HashSet<(usize, usize)> = HashSet::with_capacity(size * size);

    for row in 0..size {
        for column in 0..size {
            coordinates.insert((row, column));
        }
    }

    for (row, column) in coordinates.into_iter().take(difficulty) {
        sudoku[row][column] = 0;
    }
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
        let sudoku = generate(BoardSize::Four);
        assert!(verify(&sudoku));
    }

    #[test]
    fn test_9x9_sudoku() {
        let sudoku = generate(BoardSize::Nine);
        assert!(verify(&sudoku));
    }

    #[test]
    fn test_16x16_sudoku() {
        let sudoku = generate(BoardSize::Sixteen);
        assert!(verify(&sudoku));
    }
}
