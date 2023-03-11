use crate::commons::{COLUMN_INDEX, SQUARE_START};

pub struct Sudoku {
    pub array: [u8; 81],
    pub steps: u32,
    pub to_explore: [usize; 81],
    pub exploration_done: usize,
}

impl Sudoku {
    pub fn new(s: &str) -> Sudoku {
        // read sudoku
        let mut array = [0; 81];
        for (index, num) in s.chars().enumerate() {
            if let Some(digit) = num.to_digit(10) {
                array[index] = digit as u8;
            }
        }

        // remove clues from to_explore  : it is slower : WHY?
        let mut to_explore: [usize; 81] = core::array::from_fn(|i| i);
        let mut exploration_done = 81;
        let mut x = 0;
        while x < exploration_done {
            if array[to_explore[x]] != 0 {
                exploration_done -= 1;
                to_explore.swap(x, exploration_done);
            } else {
                x += 1;
            }
        }

        Sudoku {
            array,
            steps: 0,
            to_explore,
            exploration_done,
        }
    }

    fn add_works(&self, num: u8, index: usize) -> bool {
        let col_1d = COLUMN_INDEX[index];
        let row_1d = index - col_1d;
        for i in 0..9 {
            if (self.array[row_1d + i] == num) | (self.array[col_1d + i * 9] == num) {
                return false;
            }
        }
        let square_index = SQUARE_START[index];
        for i in 0..3 {
            let square_row = i * 9 + square_index;
            if (self.array[square_row] == num)
                | (self.array[square_row + 1] == num)
                | (self.array[square_row + 2] == num)
            {
                return false;
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        self.steps += 1;
        let Some((possibles, index_of_index_min, solved)) = self.fewest_possibilities() else {
            return false;
        };
        if solved {
            return true;
        }
        let index = self.to_explore[index_of_index_min];
        for num in possibles {
            // Try a number
            self.array[index] = num;
            self.exploration_done -= 1;
            self.to_explore
                .swap(index_of_index_min, self.exploration_done);
            if self.solve() {
                return true;
            }
            // Number didnt work, backtrack
            self.to_explore
                .swap(index_of_index_min, self.exploration_done);
            self.exploration_done += 1;
            self.array[index] = 0;
        }
        false
    }

    fn fewest_possibilities(&mut self) -> Option<(Vec<u8>, usize, bool)> {
        let mut solved = true;
        let mut min_possibilities = 10;
        let mut index_min = 100;
        let mut possibles_min = Vec::new();
        for index_of_index in 0..self.exploration_done {
            let i = self.to_explore[index_of_index];
            let possibles: Vec<u8> = (1..10)
                .filter_map(|k| self.add_works(k, i).then_some(k))
                .collect();
            let len = possibles.len();
            match len {
                0 => return None,
                1 => return Some((possibles, index_of_index, false)),
                _ => {
                    if len < min_possibilities {
                        min_possibilities = len;
                        index_min = index_of_index;
                        possibles_min = possibles;
                        solved = false;
                    }
                }
            }
        }
        Some((possibles_min, index_min, solved))
    }
}

pub fn basic_solve(sudoku: &str) -> [u8; 81] {
    let mut sol = Sudoku::new(sudoku);
    sol.solve();
    sol.array
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons::solutions;
    use sha256::digest;
    use std::error::Error;

    #[test]
    fn easy() -> Result<(), Box<dyn Error>> {
        let sudoku =
            "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....";
        let expected_sol = [
            1, 8, 3, 2, 7, 9, 4, 6, 5, 4, 6, 9, 5, 8, 3, 7, 1, 2, 2, 7, 5, 4, 6, 1, 8, 9, 3, 3, 4,
            2, 9, 5, 8, 1, 7, 6, 5, 9, 7, 1, 3, 6, 2, 8, 4, 6, 1, 8, 7, 2, 4, 3, 5, 9, 9, 5, 4, 8,
            1, 2, 6, 3, 7, 7, 3, 1, 6, 4, 5, 9, 2, 8, 8, 2, 6, 3, 9, 7, 5, 4, 1,
        ];

        let sol = basic_solve(sudoku);

        assert_eq!(expected_sol, sol);
        Ok(())
    }

    #[test]
    fn hard() -> Result<(), Box<dyn Error>> {
        let sudoku =
            "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
        let expected_sol = [
            4, 1, 7, 3, 6, 9, 8, 2, 5, 6, 3, 2, 1, 5, 8, 9, 4, 7, 9, 5, 8, 7, 2, 4, 3, 1, 6, 8, 2,
            5, 4, 3, 7, 1, 6, 9, 7, 9, 1, 5, 8, 6, 4, 3, 2, 3, 4, 6, 9, 1, 2, 7, 5, 8, 2, 8, 9, 6,
            4, 3, 5, 7, 1, 5, 7, 3, 2, 9, 1, 6, 8, 4, 1, 6, 4, 8, 7, 5, 2, 9, 3,
        ];
        let sol = basic_solve(sudoku);

        assert_eq!(expected_sol, sol);
        Ok(())
    }

    #[test]
    fn weird_case() -> Result<(), Box<dyn Error>> {
        let sudoku =
            "010300600500280000080000300071000000000700400600010205005000080000050703806004001";
        let expected_sol = [
            2, 1, 7, 3, 4, 5, 6, 9, 8, 5, 6, 3, 2, 8, 9, 1, 4, 7, 9, 8, 4, 1, 6, 7, 3, 5, 2, 4, 7,
            1, 5, 2, 6, 8, 3, 9, 3, 5, 2, 7, 9, 8, 4, 1, 6, 6, 9, 8, 4, 1, 3, 2, 7, 5, 7, 2, 5, 6,
            3, 1, 9, 8, 4, 1, 4, 9, 8, 5, 2, 7, 6, 3, 8, 3, 6, 9, 7, 4, 5, 2, 1,
        ];
        let sol = basic_solve(sudoku);

        assert_eq!(expected_sol, sol);
        Ok(())
    }

    #[test]
    fn solve_10k_sudoku_sha() -> Result<(), Box<dyn Error>> {
        let filename = "hard_sudokus.txt";
        let hash = digest(solutions(filename, basic_solve));
        let correct_hash = "b3df4de0e6f9d94b923ff2474db4da792c37e17ed4ad8dca2537fb4d65d35c83";
        assert_eq!(hash, correct_hash);
        Ok(())
    }
}
