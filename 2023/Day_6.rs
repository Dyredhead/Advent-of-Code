use std::fs;

static INPUT_FILE: &str = "Day_6.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> usize {
	let lines = fs::read_to_string(INPUT_FILE).unwrap();
	let lines: Vec<&str> = lines.lines().collect();
	let times: Vec<usize> = lines[0].split(":").collect::<Vec<&str>>()[1].split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();	
	let distances: Vec<usize> = lines[1].split(":").collect::<Vec<&str>>()[1].split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();


	let mut n = 1;
	for (time, distance) in times.into_iter().zip(distances.into_iter()) {
		let mut m = 0; 
		for i in 0..=time {
			if 1*i * (time-i) > distance {
				m+= 1;
			}
		} 
		n *= m;
	}
	return n;
}

fn pt_2() -> usize {
	let lines = fs::read_to_string(INPUT_FILE).unwrap();
	let lines: Vec<&str> = lines.lines().collect();

	let time: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1].split_ascii_whitespace().collect();
	let time: usize = time.join("").parse().unwrap();

	let distance: Vec<&str> = lines[1].split(":").collect::<Vec<&str>>()[1].split_ascii_whitespace().collect();
	let distance: usize = distance.join("").parse().unwrap();

	let mut n = 0;
	for i in 0..=time {
		if 1*i * (time-i) > distance {
			n += 1;
		}
	}
	return n;
}
