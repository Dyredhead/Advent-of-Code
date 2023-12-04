use std::fs;
use std::collections::HashMap;

static FILE_PATH: &str = "Day_1.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> i32 {
	let mut sum: i32 = 0;
	for line in fs::read_to_string(FILE_PATH).unwrap().lines() {
		let mut a: i32 = 0;
		for c in line.chars() {
			if ('0' <= c && c <= '9') {
				a = c as i32 - '0' as i32;
				break;
			} 
		}
		let mut b: i32 = 0;
		for c in line.chars().rev() {
			if ('0' <= c && c <= '9') {
				b = c as i32 - '0' as i32;
				break;
			} 
		}
		sum += a*10 + b;
	}
	return sum;
}

fn pt_2() -> i32 {
	let map = HashMap::from([
		("1", 1),
		("2", 2),
		("3", 3),
		("4", 4),
		("5", 5),
		("6", 6),
		("7", 7),
		("8", 8),
		("9", 9),
		("one", 1),
		("two", 2),
		("three", 3),
		("four", 4),
		("five", 5),
		("six", 6),
		("seven", 7),
		("eight", 8),
		("nine", 9),
	]);

	let mut sum: i32 = 0;
	for line in fs::read_to_string(FILE_PATH).unwrap().lines() {
		let mut min_key: &str = "";
		let mut min_index: i32 = line.len() as i32;
		for key in map.keys() {
			let index: i32;
			match line.find(key) {
				None => {continue;}
				Some(i) => {index = i as i32;}
			}
			if (index < min_index) {
				min_index = index;
				min_key = key;
			}
		} let a: i32 = *map.get(min_key).unwrap();
		
		let mut max_key: &str = "";
		let mut max_index: i32 = -1;
		for key in map.keys() {
			let index: i32;
			match line.rfind(key) {
				None => {continue;}
				Some(i) => {index = i as i32;}
			}
			if (index > max_index) {
				max_index = index;
				max_key = key;
			}
		} let b: i32 = *map.get(max_key).unwrap();

		sum += a*10 + b;
	}
	return sum;
}
