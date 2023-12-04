use std::fs;
use std::cmp::{min, max};

static FILE_PATH: &str = "Day_3.txt";

fn main() {
	println!("{}", pt_1());
	//println!("{}", pt_2());
}

fn pt_1() -> i32 {
	fn find_symbol(engine: &Vec<Vec<char>>) -> Option<(usize, usize)> {
		for i in (0..engine.len()) {
			for j in (0..engine[i].len()) {
				if !(('0' <= engine[i][j] && engine[i][j] <= '9') || engine[i][j] == '.') {
					return Some((i, j));
				}
			}
		}
		return None;
	}
	
	fn get_neighbors(engine: &Vec<Vec<char>>, center: (usize, usize)) -> Vec<(usize, usize)> {
		let mut neighbors: Vec<(usize, usize)> = Vec::new();
		let max_row = engine.len();
		let max_col = engine[0].len();
		let i = center.0;
		let j = center.1;
		for x in max(0, i-1)..min(i+1, max_row) {
			for y in max(0, j-1)..min(j+1, max_col) {
		    	if(x != i || y != j) {
		     		neighbors.push((y, x));
		    	}
		  	}
		}
		return neighbors; 
	}
	
	let mut engine: Vec<Vec<char>> = Vec::new();

	for line in fs::read_to_string(FILE_PATH).unwrap().lines() {
		engine.push(line.chars().collect());
	}

	let sum = 0;
	while find_symbol(&engine) != None {
		let symbol = find_symbol(&engine).unwrap();
		let neighbors = get_neighbors(&engine, symbol) ;
		for neighbor in neighbors {
			if '0' <= engine[neighbor.0][neighbor.1] && engine[neighbor.0][neighbor.1] <= '9' {
				let i = neighbor.1;
				while (0 <= i-1 && '0' <= engine[neighbor.0][i] && engine[neighbor.0][i] <= '9') {
					i--;
				}
				let j = neighbor.1
				while (j+1 < engine.len() && '0' <= engine[neighbor.0][j] && engine[neighbor.0][j] <= '9') {
					j++;
				}

				let mut number = String::new();
				for k in (i..j+1) {
					number.push_str(engine[neighbor.1][k]);
					engine[neighbor.1][k] = '.';
				}
				let number = number.parse().unwrap();
				sum += number;
			}
		}
	}
	return 0;
}
