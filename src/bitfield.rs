use crate::constants::{COLUMN_INDEX, NINE_ONES, SQUARE_START};

#[derive(Debug, Clone)]
pub struct Solver {
    /// Possible numbers in a given cell encodded by position in integer
    pub options: [u16; 81],
    /// indexes of all cells to explore, indexes after pos are done
    pub to_explore: [u8; 81],
    /// Index from which point the indexes are done as stored in data
    pub exploration_done: usize,
    // pub nb_clone: usize
}

/// Put a 0 at available positions of option at index for each cell
/// in the same row / col / square as index
fn apply_number(options: &mut [u16; 81], index: usize) {
    let value = options[index];
    let not_value = NINE_ONES - value;
    let col = COLUMN_INDEX[index];
    let row = index - col;
    let square = SQUARE_START[index];

    // row
    options[row] &= not_value;
    options[row + 1] &= not_value;
    options[row + 2] &= not_value;
    options[row + 3] &= not_value;
    options[row + 4] &= not_value;
    options[row + 5] &= not_value;
    options[row + 6] &= not_value;
    options[row + 7] &= not_value;
    options[row + 8] &= not_value;

    // col
    options[col] &= not_value;
    options[col + 9] &= not_value;
    options[col + 18] &= not_value;
    options[col + 27] &= not_value;
    options[col + 36] &= not_value;
    options[col + 45] &= not_value;
    options[col + 54] &= not_value;
    options[col + 63] &= not_value;
    options[col + 72] &= not_value;

    // square
    options[square] &= not_value;
    options[square + 1] &= not_value;
    options[square + 2] &= not_value;
    options[square + 9] &= not_value;
    options[square + 10] &= not_value;
    options[square + 11] &= not_value;
    options[square + 18] &= not_value;
    options[square + 19] &= not_value;
    options[square + 20] &= not_value;

    // put back initial value
    options[index] = value;
}


/// Check if cell has only one possibility
fn hidden_singles(options: &mut [u16; 81], index: usize) {
    let value = options[index];
    options[index] = 0;
    let col = COLUMN_INDEX[index];
    let row = index - col;
    let square = SQUARE_START[index];
    let needed = NINE_ONES
        - ((options[row]
            | options[row + 1]
            | options[row + 2]
            | options[row + 3]
            | options[row + 4]
            | options[row + 5]
            | options[row + 6]
            | options[row + 7]
            | options[row + 8])
            & (options[col]
                | options[col + 9]
                | options[col + 18]
                | options[col + 27]
                | options[col + 36]
                | options[col + 45]
                | options[col + 54]
                | options[col + 63]
                | options[col + 72])
            & (options[square]
                | options[square + 1]
                | options[square + 2]
                | options[square + 9]
                | options[square + 10]
                | options[square + 11]
                | options[square + 18]
                | options[square + 19]
                | options[square + 20]));
    match needed.count_ones() {
        0 => options[index] = value, // faster
        1 => options[index] = value & needed,
        // _ => options[index] = value,
        _ => {} // faster, keep at 0
    }
}

impl Solver {
    fn new(s: &str) -> Solver {
        let mut options = [NINE_ONES; 81];
        for (index, num) in s.chars().enumerate() {
            if let Some(digit) = num.to_digit(10) {
                if digit != 0 {
                    options[index] = 1 << (digit - 1);
                    apply_number(&mut options, index);
                }
            }
        }
        Solver {
            options,
            to_explore: core::array::from_fn(|i| i as u8),
            exploration_done: 81,
            // nb_clone: 0
        }
    }

    fn cell_done(&mut self, index: usize) {
        self.exploration_done -= 1;
        self.to_explore.swap(index, self.exploration_done);
    }

    fn process(&mut self, routes: &mut Vec<Solver>) -> bool {
        let mut values = Vec::with_capacity(9);
        loop {
            let mut min_length = 10;
            let mut min_pos: usize = 0;
            let mut min_pos_x = 0;
            // index of currently inspected cell in data
            let mut index_of_index_to_explore = 0;

            // check for hidden singles
            while index_of_index_to_explore < self.exploration_done {
                // index of currently inspected cell in options
                let pos = self.to_explore[index_of_index_to_explore];
                hidden_singles(&mut self.options, pos as usize);
                let option = self.options[pos as usize];
                let lenght = option.count_ones();
                if lenght < min_length {
                    match lenght {
                        0 => return false, // faster, more then one hidden here
                        // found hidden single
                        1 => {
                            apply_number(&mut self.options, pos as usize);
                            self.cell_done(index_of_index_to_explore);
                            // no need to increment index_of_index_to_explore because
                            // it represent a new number
                        }
                        _ => {
                            min_length = lenght;
                            min_pos = pos as usize;
                            min_pos_x = index_of_index_to_explore;
                            index_of_index_to_explore += 1;
                        }
                    }
                } else {
                    index_of_index_to_explore += 1;
                }
            }

            // Apply a number to cell with min options and push to stack the other options
            if min_length != 10 {
                let option_ = self.options[min_pos];
                values.clear();
                for i in 0..9 {
                    if option_ & 1 << i != 0 {
                        values.push(i + 1);
                    }
                }

                if values.is_empty() {
                    return false;
                }

                self.cell_done(min_pos_x);
                let item = values.pop().unwrap();

                // self.nb_clone += values.len();

                for value in &values {
                    let mut clone = self.clone();
                    clone.options[min_pos] = 1 << (value - 1);
                    apply_number(&mut clone.options, min_pos);
                    routes.push(clone);
                }
                self.options[min_pos] = 1 << (item - 1);
                apply_number(&mut self.options, min_pos);
            } else {
                return true;
            }
        }
    }

    fn get_result(&self) -> [u8; 81] {
        let mut solution = [0; 81];
        for (index, option) in self.options.iter().enumerate() {
            for i in 0..9 {
                if *option == 1 << i {
                    solution[index] = i + 1;
                }
            }
        }
        // println!("nb_clone {}", self.nb_clone);
        solution
    }
}

pub fn solve(sudoku: &str) -> [u8; 81] {
    let mut routes = vec![Solver::new(sudoku)];
    while !routes.is_empty() {
        let mut route = routes.pop().unwrap();
        let result = route.process(&mut routes);
        if result {
            return route.get_result();
        }
    }
    panic!("sapristi");
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use std::error::Error;
    use test::{black_box, Bencher};

    #[bench]
    fn solve_sudoku_bitfield_easy(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let sudoku =
            "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....";

        b.iter(|| {
            black_box(solve(sudoku));
        });

        Ok(())
    }

    #[bench]
    fn solve_sudoku_bitfield_hard(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
        let sudoku =
            "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

        b.iter(|| {
            black_box(solve(sudoku));
        });

        Ok(())
    }
}