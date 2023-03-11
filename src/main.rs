mod basic;
mod bitfield;
mod commons;

use crate::basic::basic_solve;
use crate::bitfield::bitfield_solve;
use crate::commons::solutions;

fn main() {
    // let filename = "all_17_clue_sudokus.txt";
    let filename = "hard_sudokus.txt";

    let sols = solutions(filename, bitfield_solve);
    std::fs::write("bitfield_solutions.txt", sols).ok();

    let sols = solutions(filename, basic_solve);
    std::fs::write("basic_solutions.txt", sols).ok();
}
