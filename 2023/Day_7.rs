/*
Step 1: define 'hand' struct, which contains the hand
Step 2: implement a compare function which when given two hands returns a type Ordering
Step 1: write a compare function, that given two hands, 
*/
use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

static INPUT_FILE: &str = "Day_7.txt";

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

#[derive(Debug)]
pub struct Set {
	name: String,
	hand: [usize; 5],
	bid: usize,
	stype: usize,
}

impl Ord for Set {
	fn cmp(&self, other: &Set) -> Ordering {
		return self.partial_cmp(other).unwrap();
	}
}

impl PartialOrd for Set {
	fn partial_cmp(&self, other: &Set) -> Option<Ordering> {
		if self.stype != other.stype {
			return Some(self.stype.cmp(&other.stype));
		} else {
			for (i, j) in self.hand.iter().zip(other.hand.iter()) {
				if i != j {
					return Some(i.cmp(j));
				}
			} return Some(Ordering::Equal);
		}
	}
}

impl PartialEq for Set {
	fn eq(&self, other: &Set) -> bool {
		return self.hand == other.hand;
	}
}

impl Eq for Set {}

pub struct SType {
	name: &str, 
	value: usize,
}

impl SType {
	const FiveOfAKind: 	Self 	= Self{name: "Five of a Kinds", value: 6};
	const FourOfAKind: 	Self 	= Self{name: "Four of a Kinds", value: 5};
	const FullHouse: 	Self 	= Self{name: "Full House",  value: 4};
	const ThreeOfAKind: Self 	= Self{name: "Three of a Kinds", value: 3};
	const TwoPair: 		Self 	= Self{name: "Two Pair",  value: 2};
	const OnePair: 		Self 	= Self{name: "One Pair",  value: 1};
	const HighCard: 	Self	= Self{name: "High Card",  value: 0};
}



fn pt_1() -> usize {
	fn hand_to_stype(hand: [usize; 5]) -> usize {
		let frequency: [usize; 13] = frequency(hand);
		if is_five_of_a_kind(frequency) {
			return 6;
		} else if is_four_of_a_kind(frequency) {
			return 5;
		} else if is_full_house(frequency) {
			return 4;
		} else if is_three_of_a_kind(frequency) {
			return 3;
		} else if is_two_pair(frequency) {
			return 2;
		} else if is_one_pair(frequency) {
			return 1;
		} else {
			return 0;
		}

		fn frequency(hand: [usize; 5]) -> [usize; 13] {
			let mut frequency: [usize; 13] = [0; 13];
			for i in 2..=14 {
			
				frequency[i-2] = hand.iter().filter(|n| *n == &i).count();
			}
			return frequency;
		}

		fn is_five_of_a_kind(frequency: [usize; 13]) -> bool {
			return frequency.contains(&5);
		} 
		fn is_four_of_a_kind(frequency: [usize; 13]) -> bool {
			return frequency.contains(&4);
		}
		fn is_full_house(frequency: [usize; 13]) -> bool {
			return frequency.contains(&3) && frequency.contains(&2); 
		}
		fn is_three_of_a_kind(frequency: [usize; 13]) -> bool {
			return frequency.contains(&3);
		}
		fn is_two_pair(mut frequency: [usize; 13]) -> bool {
			if frequency.contains(&2) {
				let i: usize = frequency.iter().position(|i| i == &2).unwrap();
				frequency[i] = 0;
				return frequency.contains(&2);
			}
			return false;
		}
		fn is_one_pair(frequency: [usize; 13]) -> bool {
			return frequency.contains(&2);
		}
	}



	fn hand_str_to_array(hand: &str) -> [usize; 5] {
		let map_card_to_value: HashMap<char, usize> = HashMap::from([ 
			('A', 14),
			('K', 13),
			('Q', 12),
			('J', 11),
			('T', 10),
		]);
		
		let mut hand_new: [usize; 5] = [0; 5];
		for (i, card) in hand.chars().enumerate() {
			if card.is_ascii_digit() {
				hand_new[i] = card.to_digit(10).unwrap() as usize;
			} else {
				hand_new[i] = *map_card_to_value.get(&card).unwrap();
			}
		} return hand_new;
	}

	let mut sets: Vec<Set> = Vec::new();
	for line in fs::read_to_string(INPUT_FILE).unwrap().lines() {
		let line: Vec<&str> = line.split(" ").collect();
		let name: &str = line[0];
		let hand: [usize; 5] = hand_str_to_array(name);
		let bid: usize = line[1].parse().unwrap();
		let set = Set {
						name: name.to_string(),
						hand: hand, 
						bid: bid,
						stype: hand_to_stype(hand), 
					};
		sets.push(set);
	}

	
	sets.sort();
	// for set in &sets {
		// println!("{:?}: {:?} -> {}", set.name, set.hand, set.stype);
	// }
	let mut sum: usize = 0;
	for (i, set) in sets.into_iter().enumerate() {
		sum += set.bid*(i+1);
	}
	return sum;
}


fn pt_2() -> usize {
	fn hand_to_stype(hand: [usize; 5]) -> usize {
		let mut frequency: [usize; 13] = frequency(hand);
		let jokers: usize = frequency[11-2];
		frequency[11-2] = 0;
		
		if is_five_of_a_kind(frequency, jokers) {
			return 6;
		} else if is_four_of_a_kind(frequency, jokers) {
			return 5;
		} else if is_full_house(frequency, jokers) {
			return 4;
		} else if is_three_of_a_kind(frequency, jokers) {
			return 3;
		} else if is_two_pair(frequency, jokers) {
			return 2;
		} else if is_one_pair(frequency, jokers) {
			return 1;
		} else {
			return 0;
		}

		fn frequency(hand: [usize; 5]) -> [usize; 13] {
			let mut frequency: [usize; 13] = [0; 13];
			for i in 2..=14 {
				frequency[i-2] = hand.iter().filter(|n| *n == &i).count();
			}
			return frequency;
		}

		fn is_five_of_a_kind(frequency: [usize; 13], jokers: usize) -> bool {
			return frequency.contains(&(5-jokers));
		} 
		fn is_four_of_a_kind(frequency: [usize; 13], jokers: usize) -> bool {
			return frequency.contains(&(4-jokers));
		}
		fn is_full_house(frequency: [usize; 13], jokers: usize) -> bool {
			return frequency.contains(&(3-jokers)) && frequency.contains(&2); 
		}
		fn is_three_of_a_kind(frequency: [usize; 13], jokers: usize) -> bool {
			return frequency.contains(&(3-jokers));
		}
		fn is_two_pair(mut frequency: [usize; 13], jokers: usize) -> bool {
			if frequency.contains(&(2-jokers)) {
				let i: usize = frequency.iter().position(|i| i == &(2-jokers)).unwrap();
				frequency[i] = 0;
				return frequency.contains(&2);
			}
			return false;
		}
		fn is_one_pair(frequency: [usize; 13], jokers: usize) -> bool {
			return frequency.contains(&(2-jokers));
		}
	}



	fn hand_str_to_array(hand: &str) -> [usize; 5] {
		let map_card_to_value: HashMap<char, usize> = HashMap::from([ 
			('A', 14),
			('K', 13),
			('Q', 12),
			('J', 11),
			('T', 10),
		]);
		
		let mut hand_new: [usize; 5] = [0; 5];
		for (i, card) in hand.chars().enumerate() {
			if card.is_ascii_digit() {
				hand_new[i] = card.to_digit(10).unwrap() as usize;
			} else {
				hand_new[i] = *map_card_to_value.get(&card).unwrap();
			}
		} return hand_new;
	}

	let mut sets: Vec<Set> = Vec::new();
	for line in fs::read_to_string(INPUT_FILE).unwrap().lines() {
		let line: Vec<&str> = line.split(" ").collect();
		let name: &str = line[0];
		let hand: [usize; 5] = hand_str_to_array(name);
		let bid: usize = line[1].parse().unwrap();
		let set = Set {
						name: name.to_string(),
						hand: hand, 
						bid: bid,
						stype: hand_to_stype(hand), 
					};
		sets.push(set);
	}

	
	sets.sort();
	for set in &sets {
		println!("{:?}: {:?} -> {}", set.name, set.hand, set.stype);
	}
	let mut sum: usize = 0;
	for (i, set) in sets.into_iter().enumerate() {
		sum += set.bid*(i+1);
	}
	return sum;
}
