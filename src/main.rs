#![feature(test)]
#![allow(dead_code)]

mod basic;
mod bitfield;
mod constants;

use rayon::prelude::*;

use bitfield::solve;

fn main() {
    let filename = "all_17_clue_sudokus.txt";
    // let filename = "hard_sudokus.txt";
    let sudokus = std::fs::read_to_string(filename).unwrap();
    let nb_sudoku: String = sudokus.lines().take(1).collect();
    let solutions = sudokus
        .lines()
        .skip(1)
        // .par_bridge() // doesnt preserve order => bad sha256
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|sudoku| {
            format!(
                "{},{}",
                sudoku,
                solve(sudoku)
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    std::fs::write("test.txt", nb_sudoku + "\n" + &solutions + "\n").ok();

    // let sudoku_input = "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....";
    // let sudoku_input = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    // println!("{:?}", sudoku_input);
    // let solution = solve(sudoku_input);
    // let mut sudoku = SudokuSolver::new(sudoku_input);
    // println!("{:?}", solution);
    // let solved = sudoku.solve();
    // println!("{solved}, steps={} \n{:?}", sudoku.steps, sudoku.array);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use sha256::digest;
    use std::error::Error;

    #[test]
    fn solve_10k_sudoku_bitfield_sha() -> Result<(), Box<dyn Error>> {
        let filename = "hard_sudokus.txt";
        let sudokus = std::fs::read_to_string(filename).unwrap();
        let nb_sudoku: String = sudokus.lines().take(1).collect();
        let solutions = sudokus
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .par_iter()
            .map(|sudoku| {
                format!(
                    "{},{}",
                    sudoku,
                    solve(sudoku)
                        .iter()
                        .map(ToString::to_string)
                        .collect::<String>()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        let final_file_content = nb_sudoku + "\n" + &solutions + "\n";
        let hash = digest(final_file_content);
        let correct_hash = "b3df4de0e6f9d94b923ff2474db4da792c37e17ed4ad8dca2537fb4d65d35c83";

        assert_eq!(hash, correct_hash);
        Ok(())
    }

    #[test]
    fn solve_50k_sudoku_bitfield_hard_sha() -> Result<(), Box<dyn Error>> {
        let filename = "all_17_clue_sudokus.txt";
        let sudokus = std::fs::read_to_string(filename).unwrap();
        let nb_sudoku: String = sudokus.lines().take(1).collect();
        let solutions = sudokus
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .par_iter()
            .map(|sudoku| {
                format!(
                    "{},{}",
                    sudoku,
                    solve(sudoku)
                        .iter()
                        .map(ToString::to_string)
                        .collect::<String>()
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        let final_file_content = nb_sudoku + "\n" + &solutions + "\n";
        let hash = digest(final_file_content);
        let correct_hash = "0bc8dda364db7b99f389b42383e37b411d9fa022204d124cb3c8959eba252f05";

        assert_eq!(hash, correct_hash);

        Ok(())
    }
}
