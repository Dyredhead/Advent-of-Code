use std::fs;
use std::collections::HashMap;

static INPUT_FILE: &str = "Day_8.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> usize {
	let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
	let lines: String = fs::read_to_string(INPUT_FILE).unwrap();
	let mut lines = lines.lines().collect::<Vec<_>>().into_iter();
	let directions: Vec<char> = lines.next().unwrap().chars().collect();
	lines.next();
	for line in lines {
		let line: Vec<&str> = line.split("=").collect();
		let node = line[0].trim();

		let chars: &[_] = &['(', ')'];
		let b: Vec<&str> = line[1].trim().trim_matches(chars).split(",").collect();
		let left: &str = b[0].trim();
		let right: &str = b[1].trim();

		map.insert(node, [left, right]);
	}
	// println!("{:?}", directions);
	// println!("{:?}", map);

	let mut steps: usize = 0;
	let mut node: &str = "AAA";
	while node != "ZZZ" {
		let dir: usize = if directions[steps%directions.len()] == 'L' {
							0
						} else {
							1
						};
		node = map.get(node).unwrap()[dir];
		steps += 1;
	}
	return steps;
	
}

fn pt_2() -> usize {
	let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
	let lines: String = fs::read_to_string(INPUT_FILE).unwrap();
	let mut lines = lines.lines().collect::<Vec<_>>().into_iter();
	let directions: Vec<char> = lines.next().unwrap().chars().collect();
	lines.next();
	for line in lines {
		let line: Vec<&str> = line.split("=").collect();
		let node = line[0].trim();

		let chars: &[_] = &['(', ')'];
		let b: Vec<&str> = line[1].trim().trim_matches(chars).split(",").collect();
		let left: &str = b[0].trim();
		let right: &str = b[1].trim();

		map.insert(node, [left, right]);
	}

	let nodes_start: Vec<&str> = map.keys().filter(|&node| node.chars().last().unwrap() == 'A').map(|&node| node).collect();

	// find the number of steps it takes for each 'A' node to reach a 'Z' node. The return the lcm of all of them. 
	let mut node_steps: Vec<usize> = Vec::new();
	for node_start in nodes_start {
		let mut steps: usize = 0;
		let mut node: &str = node_start;
		while node.chars().last().unwrap() != 'Z' {
			let dir: usize = if directions[steps%directions.len()] == 'L' {
				0
			} else {
				1
			};
			node = map.get(node).unwrap()[dir];
			steps += 1;
		}
		node_steps.push(steps);
	}
	return lcm(node_steps);
}

fn lcm(nums: Vec<usize>) -> usize {
	if nums.len() == 1 {
		return nums[0];
	}
	let a = nums[0];
	let b = lcm((nums[1..]).to_vec());
	return (a * b) / gcd(a, b);
}

fn gcd(a: usize, b: usize) -> usize {
	if b == 0 {
		return a;
	}
	return gcd(b, a%b);
}
