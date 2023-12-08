use std::fs;
use std::cmp::{min, max};

static FILE_PATH: &str = "Day_3.txt";

fn main() {
	// println!("{:#?}", (0..5));
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> i32 {
	fn find_symbol(engine: &Vec<Vec<char>>) -> Option<(usize, usize)> {
		for i in 0..engine.len() {
			for j in 0..engine[i].len() {
				if !(engine[i][j].is_ascii_digit() || engine[i][j] == '.' || engine[i][j] == '_') {
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
		for y in max(0, i-1)..min(i+1, max_row)+1 {
			for x in max(0, j-1)..min(j+1, max_col)+1 {
		    	if (y != i || x != j) && engine[y][x].is_ascii_digit() {
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

	let mut sum = 0;
	while find_symbol(&engine) != None {
		let symbol = find_symbol(&engine).unwrap();
		engine[symbol.0][symbol.1] = '_';

		let neighbors = get_neighbors(&engine, symbol);
		for neighbor in neighbors {
			// println!("{:?}", neighbor);
			if engine[neighbor.0][neighbor.1].is_ascii_digit() {
				let mut i = 0; 
				for _i in (0..neighbor.1).rev() { 
					if !engine[neighbor.0][_i].is_ascii_digit() {
						i = _i+1;
						break;
					}
				}
				let mut j = engine[neighbor.0].len();
				for _j in (neighbor.1)..(engine[neighbor.0].len()) { 
					if !engine[neighbor.0][_j].is_ascii_digit() {
						j = _j;
						break;
					}
				}

				let mut number = String::new();
				for k in i..j {
					number.push(engine[neighbor.0][k]);
					engine[neighbor.0][k] = '_';
				}
				// print_array_2d(&engine);
				// println!("{}", number);
				let number: usize = number.parse().unwrap();
				sum += number;
			}
		}
	}
	return sum as i32;
}

fn pt_2() -> i32 {
	fn find_symbol(engine: &Vec<Vec<char>>) -> Option<(usize, usize)> {
		for i in 0..engine.len() {
			for j in 0..engine[i].len() {
				if engine[i][j] == '*' {
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
		for y in max(0, i-1)..min(i+1, max_row)+1 {
			for x in max(0, j-1)..min(j+1, max_col)+1 {
		    	if (y != i || x != j) && engine[y][x].is_ascii_digit() {
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

	let mut sum = 0;
	while find_symbol(&engine) != None {
		let symbol = find_symbol(&engine).unwrap();
		engine[symbol.0][symbol.1] = '_';

		let neighbors = get_neighbors(&engine, symbol);
		let mut product = 1;
		let mut parts = 0;
		for neighbor in neighbors {
			// println!("{:?}", neighbor);
			if engine[neighbor.0][neighbor.1].is_ascii_digit() {
				let mut i = 0; 
				for _i in (0..neighbor.1).rev() { 
					if !engine[neighbor.0][_i].is_ascii_digit() {
						i = _i+1;
						break;
					}
				}
				let mut j = engine[neighbor.0].len();
				for _j in (neighbor.1)..(engine[neighbor.0].len()) { 
					if !engine[neighbor.0][_j].is_ascii_digit() {
						j = _j;
						break;
					}
				}

				let mut number = String::new();
				for k in i..j {
					number.push(engine[neighbor.0][k]);
					engine[neighbor.0][k] = '_';
				}
				// print_array_2d(&engine);
				// println!("{}", number);
				let number: usize = number.parse().unwrap();
				product *= number;
				parts += 1;
			} 
		}
		if parts == 2{
			sum += product;
		}
		
	}
	return sum as i32;
}
