use std::fs;
use std::cmp::{min, max};
use std::ops::Range;
static FILE_PATH: &str = "Day_5.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
} 

fn pt_1() -> usize {
	let lines_1 = fs::read_to_string(FILE_PATH).unwrap();
	let lines: Vec<&str> = lines_1.lines().collect();

	let mut seeds: Vec<usize> = lines[0]["seeds: ".len()..]
										.split_ascii_whitespace()
										.map(|x| x.parse().unwrap())
										.collect();

	let mut maps: Vec<Vec<Vec<usize>>> = Vec::new();

	let mut i = 0;
	let mut line_iter = lines.into_iter().skip(1);

	while let Some(next) = line_iter.next() {
		if next == "" {
			i += 1;
			line_iter.next();
			maps.push(Vec::new());
		} else {
			let map = next.split(" ")
							.map(|x| x.parse().unwrap())
							.collect(); 
			maps[i-1].push(map);
		}
	}

	// println!("{:?}", maps);
	// for i in &maps {
		// for j in i {
			// println!("{:?}", j);
		// }
		// println!("--------");
	// }

	for map_type in maps {
		for i in 0..seeds.len() {			
			for map in &map_type {
				let seed = seeds[i];
			
				let destination = map[0];
				let source = map[1];
				let range = map[2];

				if source <= seed && seed - source <= range - 1 {
					seeds[i] = destination + (seed - source);
					break;
				} 
			}
		}
	}
	
	return *seeds.iter().min().unwrap();
}

fn pt_2() -> usize {
	let lines = fs::read_to_string(FILE_PATH).unwrap();
	let lines: Vec<&str> = lines.lines().collect();

	let mut seeds: Vec<Range<_>> = lines[0]["seeds: ".len()..]
										.split_ascii_whitespace()
										.map(|x| x.parse().unwrap())
										.collect::<Vec<_>>()
										.windows(2)
										.map(|x| x[0]..x[0]+x[1])
										.collect();

	let mut maps: Vec<Vec<[Range<usize>; 2]>> = Vec::new();

	let mut i = 0;
	let mut line_iter = lines.into_iter().skip(1);

	while let Some(next) = line_iter.next() {
		if next == "" {
			i += 1;
			line_iter.next();
			maps.push(Vec::<[std::ops::Range<usize>; 2]>::new());
		} else {
			let map: Vec<usize> = next.split(" ")
							.map(|x| x.parse().unwrap())
							.collect();
			
			let map: [std::ops::Range<usize>; 2] = [(map[0]..map[0]+map[3]),(map[1]..map[1]+map[3])];
			maps[i-1].push(map);
		}
	}

	// println!("{:?}", maps);
	// for i in &maps {
		// for j in i {
			// println!("{:?}", j);
		// }
		// println!("--------");
	// }

	fn contains(a: Range<usize>, b: Range<usize>) -> bool {
		return max(a.start, b.start) <= min(a.end, b.end);
	} 

	for map_type in maps {
		for i in 0..seeds.len() {			
			for map in &map_type {
				// let seed = &seeds[i*2];

				
				// if max <= min && max - min {
				// 	seeds[i] = destination + (seed - source);
				// 	break;
				// } 
			}
		}
	}
	
	let mut min = seeds[0].start;
	for i in seeds {
		if i.start < min { 
			min = i.start;
		}
	}
	return min;
}
