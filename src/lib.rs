use crate::Solution::{DeadEnd, Solved};
use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Int8Array;
use js_sys::JsString;

type Field<T> = [T; 81];

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Solution {
	DeadEnd,
	Solved(Field<i8>),
}

const QUADRANT_INDICES: [usize; 81] = [
	0, 0, 0, 1, 1, 1, 2, 2, 2,
	0, 0, 0, 1, 1, 1, 2, 2, 2,
	0, 0, 0, 1, 1, 1, 2, 2, 2,
	3, 3, 3, 4, 4, 4, 5, 5, 5,
	3, 3, 3, 4, 4, 4, 5, 5, 5,
	3, 3, 3, 4, 4, 4, 5, 5, 5,
	6, 6, 6, 7, 7, 7, 8, 8, 8,
	6, 6, 6, 7, 7, 7, 8, 8, 8,
	6, 6, 6, 7, 7, 7, 8, 8, 8,
];

const QUADRANTS: [[usize; 9]; 9] = [
	[
		0, 1, 2,
		9, 10, 11,
		18, 19, 20
	],
	[
		3, 4, 5,
		12, 13, 14,
		21, 22, 23
	],
	[
		6, 7, 8,
		15, 16, 17,
		24, 25, 26
	],
	[
		27, 28, 29,
		36, 37, 38,
		45, 46, 47
	],
	[
		30, 31, 32,
		39, 40, 41,
		48, 49, 50
	],
	[
		33, 34, 35,
		42, 43, 44,
		51, 52, 53
	],
	[
		54, 55, 56,
		63, 64, 65,
		72, 73, 74
	],
	[
		57, 58, 59,
		66, 67, 68,
		75, 76, 77
	],
	[
		60, 61, 62,
		69, 70, 71,
		78, 79, 80
	],
];

const NUMBERS: [i8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn row_for_pos(pos: usize) -> usize {
	pos / 9
}

fn col_for_pos(pos: usize) -> usize {
	pos % 9
}

fn quadrant_for_pos(pos: usize) -> [usize; 9] {
	QUADRANTS[QUADRANT_INDICES[pos]]
}

fn constraints_for_part<F>(field: &Field<i8>, index_function: F) -> [i8;9] where F: Fn(usize) -> usize {
	let mut constraints = NUMBERS;

	for i in 0..9 {
		let field_index = index_function(i);
		let value = field[field_index];

		if value != -1 {
			let index: usize = (value - 1) as usize;

			constraints[index] = -1;
		}
	}

	constraints
}

fn constraints_for_row(row: usize, field: &Field<i8>) -> [i8;9] {
	constraints_for_part(&field, |i| i + row * 9)
}

fn constraints_for_col(col: usize, field: &Field<i8>) -> [i8;9] {
	constraints_for_part(&field, |i| i * 9 + col)
}

fn constraints_for_quadrant(quad: &[usize; 9], field: &Field<i8>) -> [i8;9] {
	constraints_for_part(&field, &|i| quad[i])
}

fn constraints_for_pos(pos: usize, field: &Field<i8>) -> [i8;9] {
	let row_constraints = constraints_for_row(row_for_pos(pos), &field);
	let col_constraints = constraints_for_col(col_for_pos(pos), &field);
	let quad_constraints = constraints_for_quadrant(&quadrant_for_pos(pos), &field);

	let mut constraints: [i8; 9] = [-1; 9];

	for i in 0..9 {


		if row_constraints[i] != -1 && col_constraints[i] != -1 && quad_constraints[i] != -1 {
			constraints[i] = row_constraints[i]
		}
	}

	constraints
}

fn solve_impl(field: &Field<i8>, mut result: Field<i8>, pos: usize) -> Solution {
	let len = field.len();

	if pos > len {
		return DeadEnd;
	}

	if pos == len {
		return Solved(result);
	}

	let mut constraints: [i8; 9] = [-1; 9];

	if field[pos] != -1 {
		let index: usize = ( field[pos] - 1) as usize;
		constraints[index] = field[pos];
	} else {
		constraints = constraints_for_pos(pos, &result);
	}

	for constraint in constraints {
		if constraint == -1 {
			continue;
		}

		result[pos] = constraint;

		let res = solve_impl(&field, result, pos + 1);

		if res != DeadEnd {
			return res;
		}
	}

	return DeadEnd;
}

#[wasm_bindgen]
pub fn solve(field: &Int8Array) -> Result<Int8Array, JsString> {
	if field.length() != 81 {
		return Result::Err(JsString::from("field has wrong size"));
	}

	let mut field_copy: Field<i8> = [-1; 81];

	field.copy_to(&mut field_copy);

	let res = solve_impl(&field_copy, field_copy, 0);

	match res {
		DeadEnd => Result::Err(JsString::from("field is unsolvable")),
		Solved(solution) => Result::Ok(Int8Array::from(solution.as_ref()))
	}
}

#[cfg(test)]
mod tests {
	use crate::solve_impl;

	const FIELD_EASY: crate::Field<i8> = [
		9, 4, 5, -1, -1, 8, -1, -1, 6,
		2, -1, 3, -1, 6, -1, -1, -1, 5,
		-1, -1, -1, 5, 4, 7, -1, 3, 2,
		7, -1, -1, -1, -1, 3, 2, 6, 9,
		3, -1, 4, -1, -1, 2, -1, -1, -1,
		-1, -1, 6, -1, 1, 9, 8, 4, -1,
		-1, -1, -1, 8, -1, -1, 5, 7, 1,
		6, 8, -1, -1, -1, -1, -1, -1, -1,
		-1, 5, -1, 3, 2, -1, -1, -1, 8,
	];

	#[test]
	fn test_row() {
		assert_eq!(crate::row_for_pos(0), 0);
		assert_eq!(crate::row_for_pos(8), 0);
		assert_eq!(crate::row_for_pos(9), 1);
		assert_eq!(crate::row_for_pos(18), 2);
		assert_eq!(crate::row_for_pos(22), 2);
		assert_eq!(crate::row_for_pos(80), 8);
	}

	#[test]
	fn test_column() {
		assert_eq!(crate::col_for_pos(0), 0);
		assert_eq!(crate::col_for_pos(9), 0);
		assert_eq!(crate::col_for_pos(1), 1);
		assert_eq!(crate::col_for_pos(19), 1);
		assert_eq!(crate::col_for_pos(22), 4);
		assert_eq!(crate::col_for_pos(70), 7);
		assert_eq!(crate::col_for_pos(80), 8);
	}

	#[test]
	fn test_quadrant() {
		assert_eq!(crate::quadrant_for_pos(0), [0, 1, 2, 9, 10, 11, 18, 19, 20]);
		assert_eq!(crate::quadrant_for_pos(45), [27, 28, 29, 36, 37, 38, 45, 46, 47]);
		assert_eq!(crate::quadrant_for_pos(80), [60, 61, 62, 69, 70, 71, 78, 79, 80]);
	}

	#[test]
	fn test_row_constraint() {
		let test_set_0: [i8; 9] = [1, 2, 3, -1, -1, -1, 7, -1, -1];
		let res_set_0 = crate::constraints_for_row(0, &FIELD_EASY);

		let test_set_1: [i8; 9] = [1, -1, -1, 4, 5, -1, -1, 8, -1];
		let res_set_1 = crate::constraints_for_row(3, &FIELD_EASY);

		let test_set_2: [i8; 9] = [1, -1, -1, 4, -1, 6, 7, -1, 9];
		let res_set_2 = crate::constraints_for_row(8, &FIELD_EASY);

		assert_eq!(res_set_0, test_set_0);
		assert_eq!(res_set_1, test_set_1);
		assert_eq!(res_set_2, test_set_2);
	}

	#[test]
	fn test_col_constraint() {
		let test_set_0: [i8; 9] = [1, -1, -1, 4, 5, -1, -1, 8, -1];
		let res_set_0 = crate::constraints_for_col(0, &FIELD_EASY);

		let test_set_1: [i8; 9] = [1, 2, -1, 4, -1, 6, 7, -1, 9];
		let res_set_1 = crate::constraints_for_col(3, &FIELD_EASY);

		let test_set_2: [i8; 9] = [-1, -1, 3, 4, -1, -1, 7, -1, -1];
		let res_set_2 = crate::constraints_for_col(8, &FIELD_EASY);

		assert_eq!(res_set_0, test_set_0);
		assert_eq!(res_set_1, test_set_1);
		assert_eq!(res_set_2, test_set_2);
	}

	#[test]
	fn test_quad_constraint() {
		let test_set_0: [i8; 9] = [1, -1, -1, -1, -1, 6, 7, 8, -1];
		let res_set_0 = crate::constraints_for_quadrant(&crate::QUADRANTS[0], &FIELD_EASY);

		let test_set_1: [i8; 9] = [1, 2, -1, -1, 5, -1, -1, 8, 9];
		let res_set_1 = crate::constraints_for_quadrant(&crate::QUADRANTS[3], &FIELD_EASY);

		let test_set_2: [i8; 9] = [-1, 2, 3, 4, -1, 6, -1, -1, 9];
		let res_set_2 = crate::constraints_for_quadrant(&crate::QUADRANTS[8], &FIELD_EASY);

		assert_eq!(res_set_0, test_set_0);
		assert_eq!(res_set_1, test_set_1);
		assert_eq!(res_set_2, test_set_2);
	}

	#[test]
	fn test_solve() {
		let res = solve_impl(&FIELD_EASY, FIELD_EASY, 0);

		assert_ne!(res, crate::DeadEnd);
	}
}
