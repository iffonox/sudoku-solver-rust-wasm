# sudoku-solver-rust-wasm
A simple Rust Sudoku solver.
This was just a training exercise for getting into Rust.

## Data structures
The field is represented as an Int8Array of 81 integers.
-1 represents an empty cell.

## Compiling
Build with `wasm-pack build  --target web`

## Example
````javascript
import * as Solver from "./pkg/sudoku.js";

const field = [
    9, 4, 5, -1, -1, 8, -1, -1, 6,
    2, -1, 3, -1, 6, -1, -1, -1, 5,
    -1, -1, -1, 5, 4, 7, -1, 3, 2,

    7, -1, -1, -1, -1, 3, 2, 6, 9,
    3, -1, 4, -1, -1, 2, -1, -1, -1,
    -1, -1, 6, -1, 1, 9, 8, 4, -1,

    -1, -1, -1, 8, -1, -1, 5, 7, 1,
    6, 8, -1, -1, -1, -1, -1, -1, -1,
    -1, 5, -1, 3, 2, -1, -1, -1, 8,
]

(async () => {
    await Solver.init();
    
	const result = Solver.solve(Int8Array.from(field));

	console.log(result);
})
````
