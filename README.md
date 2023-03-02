# Yet Another Sudoku Solver (in rust)

This is a sudoku solver. I did two implementations. A [basic](src/basic.rs) one using integers to represents possible numbers for each cell and a [bitfield](src/bitfield.rs) that use one integer to represent all the posibilities of one cell, where each bit represent a different number. The bitfield version is ~20 X faster.