// #![feature(iter_map_windows)]

use std::fs;
// use std::collections::HashMap;

static INPUT_FILE: &str = "Day_9.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> i32 {
	let lines: String = fs::read_to_string(INPUT_FILE).unwrap();
	let lines: Vec<&str> = lines.lines().collect::<Vec<_>>();
	let lines: Vec<Vec<&str>> = lines.iter()
										.map(|str| {
											str.split(" ").collect()
										})
										.collect();
	let histories: Vec<Vec<i32>> = lines.iter()
											.map(|x| {
												x.iter()
												.map(|y| y.parse().unwrap())
												.collect()
											})
											.collect();
	let mut sum: i32 = 0; 
	for mut history in histories {
		sum += next_value(&mut history)
	}
	return sum;
}

fn next_value(history_old: &mut Vec<i32>) -> i32 {
	if history_old.iter().all(|&x| x == 0) {
		history_old.push(0);
		return *history_old.last().unwrap();
	} else {
		let mut history_new: Vec<i32> = history_old.windows(2).map(|x| x[1] - x[0]).collect();
		next_value(&mut history_new);
		history_old.push(*history_old.last().unwrap() + *history_new.last().unwrap());
		return *history_old.last().unwrap();
	}
}

fn pt_2() -> i32 {
	let lines: String = fs::read_to_string(INPUT_FILE).unwrap();
	let lines: Vec<&str> = lines.lines().collect::<Vec<_>>();
	let lines: Vec<Vec<&str>> = lines.iter()
										.map(|str| {
											str.split(" ").collect()
										})
										.collect();
	let histories: Vec<Vec<i32>> = lines.iter()
											.map(|x| {
												x.iter()
												.map(|y| y.parse().unwrap())
												.collect()
											})
											.collect();
	let mut sum: i32 = 0; 
	for mut history in histories {
		sum += previous_value(&mut history)
	}
	return sum;
}

fn previous_value(history_old: &mut Vec<i32>) -> i32 {
	if history_old.iter().all(|&x| x == 0) {
		history_old.insert(0, 0);
		return *history_old.first().unwrap();
	} else {
		let mut history_new: Vec<i32> = history_old.windows(2).map(|x| x[1] - x[0]).collect();
		previous_value(&mut history_new);
		history_old.insert(0, *history_old.first().unwrap() - *history_new.first().unwrap());
		return *history_old.first().unwrap();
	}
}
