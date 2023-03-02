use crate::commons::{COLUMN_INDEX, SQUARE_START};

pub struct Sudoku {
    pub array: [u8; 81],
    pub steps: u32,
    pub to_explore: [usize; 81],
    pub exploration_done: usize,
}

impl Sudoku {
    pub fn new(s: &str) -> Sudoku {
        let mut array = [0; 81];
        for (index, num) in s.chars().enumerate() {
            if let Some(digit) = num.to_digit(10) {
                array[index] = digit as u8;
            }
        }
        Sudoku {
            array,
            steps: 0,
            to_explore: core::array::from_fn(|i| i),
            exploration_done: 81,
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
            self.array[index] = num;
            self.exploration_done -= 1;
            self.to_explore
                .swap(index_of_index_min, self.exploration_done);
            if self.solve() {
                return true;
            }
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
            if self.array[i] == 0 {
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
    extern crate test;
    use super::*;
    use crate::commons::solutions;
    use sha256::digest;
    use std::error::Error;
    use test::{black_box, Bencher};

    #[bench]
    fn sove_sudoku_easy(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let sudoku =
            "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....";

        b.iter(|| {
            black_box(Sudoku::new(sudoku).solve());
        });

        Ok(())
    }

    #[bench]
    fn sove_sudoku_basic_hard(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let sudoku =
            "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

        b.iter(|| {
            black_box(Sudoku::new(sudoku).solve());
        });

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

    // // very slow
    // // test basic::tests::solve_10k_sudoku       ... bench: 220,116,897 ns/iter (+/- 29,814,870)
    // #[bench]
    // fn solve_10k_sudoku(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    //     let filename = "hard_sudokus.txt";
    //     b.iter(|| {
    //         black_box(solutions(filename, basic_solve));
    //     });

    //     Ok(())
    // }

    // // TOO SLOW
    // #[test]
    // fn solve_50k_sudoku_sha() -> Result<(), Box<dyn Error>> {
    //     let filename = "all_17_clue_sudokus.txt";
    //     let hash = digest(solutions(filename, basic_solve));
    //     let correct_hash = "0bc8dda364db7b99f389b42383e37b411d9fa022204d124cb3c8959eba252f05";

    //     assert_eq!(hash, correct_hash);

    //     Ok(())
    // }
}
