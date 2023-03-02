#![feature(test)]

mod basic;
mod bitfield;
mod commons;

use crate::basic::basic_solve;
use crate::bitfield::bitfield_solve;
use crate::commons::solutions;

fn main() {
    let filename = "all_17_clue_sudokus.txt";
    // let filename = "hard_sudokus.txt";
    let sols = solutions(filename, bitfield_solve);
    // let sols = solutions(filename, basic_solve);
    std::fs::write("solutions.txt", sols).ok();

    // let sudoku_input = "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....";
    let sudoku_input =
        "000000010400000000020000000000050407008000300001090000300400200050100000000806000";
    // let sudoku_input = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    println!("{:?}", sudoku_input);
    // let solution = solve(sudoku_input);
    // bitfield version
    println!("bitfield\n{:?}", bitfield::bitfield_solve(sudoku_input));
    // basic version
    let mut sudoku = basic::Sudoku::new(sudoku_input);
    let solved = sudoku.solve();
    println!(
        "basic\n{solved}, steps={} \n{:?}",
        sudoku.steps, sudoku.array
    );

    let sudokus = std::fs::read_to_string(filename).unwrap();
    let sudoku = sudokus.lines().skip(1).take(1).collect::<Vec<&str>>()[0];
    println!(
        "equal = {} ?",
        bitfield_solve(sudoku) == basic_solve(sudoku)
    );
    println!("basic ={:?}", basic_solve(sudoku));
    println!("field ={:?}", bitfield_solve(sudoku));
}
