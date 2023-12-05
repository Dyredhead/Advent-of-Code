use std::fs;
use std::cmp::{min};

static FILE_PATH: &str = "Day_4.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

fn pt_1() -> usize {
    let mut points = 0;
    for line in fs::read_to_string(FILE_PATH).unwrap().lines() {
        let line: Vec<_> = line.split(":")
                        .collect::<Vec<_>>()[1]
                        .split("|")
                        .collect();
        
        let numbers_winning: Vec<_> = line[0].split_ascii_whitespace().collect();
        let numbers_yours: Vec<_> = line[1].split_ascii_whitespace().collect();  
        let mut matches = 0;
        for num in numbers_winning {
            if numbers_yours.contains(&num) {
                matches += 1;
            }
        }
        if matches != 0 {
            points += usize::pow(2, matches - 1);
        }
    } 
    return points;
}

fn pt_2() -> usize {
    let binding = fs::read_to_string(FILE_PATH).unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let mut card_count = 0;
    let mut card_counts = vec![1; lines.len()];
    for (i, line) in lines.clone().into_iter().enumerate() {
        card_count += card_counts[i];
        let line: Vec<_> = line.split(":")
                            .collect::<Vec<_>>()[1]
                            .split("|")
                            .collect();
        let numbers_winning: Vec<_> = line[0].split_ascii_whitespace().collect();
        let numbers_yours: Vec<_> = line[1].split_ascii_whitespace().collect();
        let mut matches = 0;
        for num in numbers_winning {
            if numbers_yours.contains(&num) {
                matches += 1;
            }
        }
        for j in i+1..min(i+matches+1, lines.len()) {
            card_counts[j] += card_counts[i];
        }
    }
    return card_count;
}
