use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub enum Error {
    InvalidNumber,
    InvalidCoordinate,
    StartingNumbers,
    UnkownDifficulty(String),
    UknownBoardSize(String),
}

// The number indicate the percentage of total number to be removed, the difficulty scale up with size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Beginner = 15,
    Easy = 30,
    #[default]
    Medium = 45,
    Hard = 57,
    Extreme = 66,
    Impossible = 79,
}

impl FromStr for Difficulty {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "beginner" => Ok(Self::Beginner),
            "easy" => Ok(Self::Easy),
            "medium" => Ok(Self::Medium),
            "hard" => Ok(Self::Hard),
            "extreme" => Ok(Self::Extreme),
            "impossible" => Ok(Self::Impossible),
            s => Err(Error::UnkownDifficulty(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardSize {
    Four = 4,
    #[default]
    Nine = 9,
    Sixteen = 16,
}

impl FromStr for BoardSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4" => Ok(Self::Four),
            "9" => Ok(Self::Nine),
            "16" => Ok(Self::Sixteen),
            s => Err(Error::UknownBoardSize(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
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

pub fn sudoku_as_string(sudoku: &Vec<Vec<u8>>) -> String {
    let size = sudoku.len();
    let size_sqrt = (size as f64).sqrt() as usize;

    let mut line_separator = (0..(if size / 10 == 0 {size + size_sqrt} else {size + size_sqrt - size / 10}))
        .map(|_| if size / 10 == 0 { " -" } else { " --" })
        .collect::<Vec<&str>>()
        .join("");
    line_separator.push_str("\n");

    let mut output: String = String::new();
    output.push_str(&line_separator);

    for row in 0..size {
        let mut row_string: String = String::new();

        for column in 0..size {
            if (column + 1) % size_sqrt != 0 {
                row_string.push_str(" ");

                if sudoku[row][column] > 0 && sudoku[row][column] < 10 && size / 10 == 0
                    || sudoku[row][column] > 10
                {
                    row_string.push_str(&sudoku[row][column].to_string())
                } else if sudoku[row][column] > 0 && sudoku[row][column] < 10 {
                    row_string.push_str(" ");
                    row_string.push_str(&sudoku[row][column].to_string())
                } else if size / 10 == 0 {
                    row_string.push_str("_")
                } else {
                    row_string.push_str("__")
                };
            } else {
                row_string.push_str(" ");

                if sudoku[row][column] > 0 && sudoku[row][column] < 10 && size / 10 == 0
                    || sudoku[row][column] > 10
                {
                    row_string.push_str(&sudoku[row][column].to_string())
                } else if sudoku[row][column] > 0 && sudoku[row][column] < 10 {
                    row_string.push_str(" ");
                    row_string.push_str(&sudoku[row][column].to_string())
                } else if size / 10 == 0 {
                    row_string.push_str("_")
                } else {
                    row_string.push_str("__")
                };
                row_string.push_str(" |");
            }
        }
        if (row + 1) % size_sqrt == 0 {
            row_string.push_str("\n");
            row_string.push_str(&line_separator);
        } else {
            row_string.push_str("\n");
        }
        output.push_str(&row_string)
    }
    output
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
