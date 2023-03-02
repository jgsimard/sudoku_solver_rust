use rayon::prelude::*;

pub const NINE_ONES: u16 = 511; // 0000000111111111 or 0x1FF

#[rustfmt::skip]
pub const COLUMN_INDEX: [usize; 81] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
    0, 1, 2, 3, 4, 5, 6, 7, 8,
];

// let square_index = index / 27 * 27 + col_1d / 3 * 3;
#[rustfmt::skip]
pub const SQUARE_START : [usize; 81] = [
    0, 0, 0, 3, 3, 3, 6, 6, 6,
    0, 0, 0, 3, 3, 3, 6, 6, 6,
    0, 0, 0, 3, 3, 3, 6, 6, 6,
    27, 27, 27, 30, 30, 30, 33, 33, 33,
    27, 27, 27, 30, 30, 30, 33, 33, 33,
    27, 27, 27, 30, 30, 30, 33, 33, 33,
    54, 54, 54, 57, 57, 57, 60, 60, 60,
    54, 54, 54, 57, 57, 57, 60, 60, 60,
    54, 54, 54, 57, 57, 57, 60, 60, 60
];

pub fn solutions(filename: &str, fn_solve: fn(&str) -> [u8; 81]) -> String {
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
                fn_solve(sudoku)
                    .iter()
                    .map(ToString::to_string)
                    .collect::<String>()
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    nb_sudoku + "\n" + &solutions + "\n"
}
