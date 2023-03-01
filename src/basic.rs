use crate::constants::{COLUMN_INDEX, ROW_INDEX, SQUARE_START};

struct Sudoku {
    pub array: [u8; 81],
    steps: u32,
}

impl Sudoku {
    fn new(s: &str) -> Sudoku {
        let mut array = [0; 81];
        for (index, num) in s.chars().enumerate() {
            if let Some(digit) = num.to_digit(10) {
                array[index] = digit as u8;
            }
        }
        Sudoku { array, steps: 0 }
    }

    fn add_works(&self, num: u8, index: usize) -> bool {
        let col = COLUMN_INDEX[index];
        let row = index - col;
        let square = SQUARE_START[index];

        if (self.array[row] == num)
            | (self.array[row + 1] == num)
            | (self.array[row + 2] == num)
            | (self.array[row + 3] == num)
            | (self.array[row + 4] == num)
            | (self.array[row + 5] == num)
            | (self.array[row + 6] == num)
            | (self.array[row + 7] == num)
            | (self.array[row + 8] == num)
        {
            return false;
        }
        match ROW_INDEX[index] {
            0 | 3 | 6 => {
                if (self.array[square + 9] == num)
                    | (self.array[square + 10] == num)
                    | (self.array[square + 11] == num)
                    | (self.array[square + 18] == num)
                    | (self.array[square + 19] == num)
                    | (self.array[square + 20] == num)
                {
                    return false;
                }
            }
            1 | 4 | 7 => {
                if (self.array[square] == num)
                    | (self.array[square + 1] == num)
                    | (self.array[square + 2] == num)
                    | (self.array[square + 18] == num)
                    | (self.array[square + 19] == num)
                    | (self.array[square + 20] == num)
                {
                    return false;
                }
            }
            2 | 5 | 8 => {
                if (self.array[square] == num)
                    | (self.array[square + 1] == num)
                    | (self.array[square + 2] == num)
                    | (self.array[square + 9] == num)
                    | (self.array[square + 10] == num)
                    | (self.array[square + 11] == num)
                {
                    return false;
                }
            }
            _ => {}
        }

        match ROW_INDEX[index] {
            0 | 1 | 2 => {
                if (self.array[col + 27] == num)
                    | (self.array[col + 36] == num)
                    | (self.array[col + 45] == num)
                    | (self.array[col + 54] == num)
                    | (self.array[col + 63] == num)
                    | (self.array[col + 72] == num)
                {
                    return false;
                }
            }
            3 | 4 | 5 => {
                if (self.array[col] == num)
                    | (self.array[col + 9] == num)
                    | (self.array[col + 18] == num)
                    | (self.array[col + 54] == num)
                    | (self.array[col + 63] == num)
                    | (self.array[col + 72] == num)
                {
                    return false;
                }
            }
            6 | 7 | 8 => {
                if (self.array[col] == num)
                    | (self.array[col + 9] == num)
                    | (self.array[col + 18] == num)
                    | (self.array[col + 27] == num)
                    | (self.array[col + 36] == num)
                    | (self.array[col + 45] == num)
                {
                    return false;
                }
            }
            _ => {}
        }
        true
    }

    // fn add_works(&self, num: u8, index: usize) -> bool {
    //     let col = COLUMN_INDEX[index];
    //     let row = index - col;
    //     let square = SQUARE_START[index];

    //     if (self.array[row] == num)Sudoku num)
    //         | (self.array[row + 8] == num)
    //         | (self.array[col] == num)
    //         | (self.array[col + 9] == num)
    //         | (self.array[col + 18] == num)
    //         | (self.array[col + 27] == num)
    //         | (self.array[col + 36] == num)
    //         | (self.array[col + 45] == num)
    //         | (self.array[col + 54] == num)
    //         | (self.array[col + 63] == num)
    //         | (self.array[col + 72] == num)
    //         | (self.array[square] == num)
    //         | (self.array[square + 1] == num)
    //         | (self.array[square + 2] == num)
    //         | (self.array[square + 9] == num)
    //         | (self.array[square + 10] == num)
    //         | (self.array[square + 11] == num)
    //         | (self.array[square + 18] == num)
    //         | (self.array[square + 19] == num)
    //         | (self.array[square + 20] == num)
    //     {
    //         return false;
    //     }
    //     true
    // }

    // fn add_works(&self, num: u8, index: usize) -> bool {
    //     let col_1d = COLUMN_INDEX[index];
    //     let row_1d = index - col_1d;

    //     for i in 0..9 {
    //         if (self.array[row_1d + i] == num) | (self.array[col_1d + i * 9] == num) {
    //             return false;
    //         }
    //     }

    //     let square_index = SQUARE_START[index];
    //     // let square_index = index / 27 * 27 + col_1d / 3 * 3;
    //     for i in 0..3 {
    //         let square_row = i * 9 + square_index;
    //         if (self.array[square_row] == num)
    //             | (self.array[square_row + 1] == num)
    //             | (self.array[square_row + 2] == num)
    //         {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn solve(&mut self) -> bool {
        self.steps += 1;
        let Some((possibles, index, solved)) = self.fewest_possibilities() else {
            return false;
        };
        if solved {
            return true;
        }
        for num in possibles {
            self.array[index] = num;
            if self.solve() {
                return true;
            }
            self.array[index] = 0;
        }
        false
    }

    fn fewest_possibilities(&mut self) -> Option<(Vec<u8>, usize, bool)> {
        let mut solved = true;
        let mut min_possibilities = 10;
        let mut index = 100;
        let mut possibles_min = Vec::new();
        for i in 0..81 {
            if self.array[i] == 0 {
                let possibles: Vec<u8> = (1..10)
                    .map(|k| (k, self.add_works(k, i)))
                    .filter(|(_, p)| *p)
                    .map(|(k, _)| k)
                    .collect();
                let len = possibles.len();
                match len {
                    0 => return None,
                    1 => return Some((possibles, i, false)),
                    2.. => {
                        if len < min_possibilities {
                            min_possibilities = len;
                            index = i;
                            possibles_min = possibles;
                            solved = false;
                        }
                    }
                    _ => {}
                }
            }
        }
        Some((possibles_min, index, solved))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use std::error::Error;
    use test::{black_box, Bencher};

    #[bench]
    fn sove_sudoku_basic_easy(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
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
}
