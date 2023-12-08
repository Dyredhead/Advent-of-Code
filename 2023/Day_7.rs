use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::atomic;

static INPUT_FILE: &str = "Day_7.txt";
static WITH_JOKERS: atomic::AtomicBool = atomic::AtomicBool::new(false);

fn main() {
	println!("{}", pt_1());
	println!("{}", pt_2());
}

#[derive(Debug, Eq)]
pub struct Set {
	hand: Hand,
	bid: usize,
}
impl Set {
	pub fn new(name: String, bid: usize) -> Self {
		return Self { 
			hand: Hand::new(name),
			bid: bid,
		}
	}
}
impl Ord for Set {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.partial_cmp(other).unwrap();
	}
}
impl PartialOrd for Set {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return Some(self.hand.cmp(&other.hand));
	}
}
impl PartialEq for Set {
	fn eq(&self, other: &Self) -> bool {
		return self.hand == other.hand;
	}
}


#[derive(Debug, Eq)]
pub struct Hand {
	name: String,
	cards: [usize; 5],
	htype: HType,
}
impl Hand {
	// const with_jokers: bool = true;
	//TODO: refactor 
	fn name_to_cards(name: &String) -> [usize; 5] {
		let map_card_to_value: HashMap<char, usize> = HashMap::from([ 
			('A', 14),
			('K', 13),
			('Q', 12),
			if WITH_JOKERS.load(atomic::Ordering::Relaxed) {
				('J', 1)
			} else {
				('J', 11)
			},
			('T', 10),
		]);
		
		let mut cards: [usize; 5] = [0; 5];
		for (i, card) in name.chars().enumerate() {
			if card.is_ascii_digit() {
				cards[i] = card.to_digit(10).unwrap() as usize;
			} else {
				cards[i] = *map_card_to_value.get(&card).unwrap();
			}
		} return cards;
	}
	fn frequency(hand: [usize; 5]) -> [usize; 14] {
		let mut frequency: [usize; 14] = [0; 14];
		for i in 1..=14 {
			frequency[i-1] = hand.iter().filter(|n| *n == &i).count();
		} return frequency;
	}

	fn cards_to_htype(cards: [usize; 5]) -> HType {
		let mut frequency: [usize; 14] = Self::frequency(cards);
		let jokers: usize = if WITH_JOKERS.load(atomic::Ordering::Relaxed) {
								let jokers: usize = frequency[1-1];
								frequency[1-1] = 0;
								jokers
							} else {
								0
							};
		if frequency.contains(&(5-jokers)) {
			return HType::FIVE_OF_A_KIND;
		} else if frequency.contains(&(4-jokers)) {
			return HType::FOUR_OF_A_KIND;
		} else if is_full_house(frequency, jokers) {
			return HType::FULL_HOUSE;
		} else if frequency.contains(&(3-jokers)) {
			return HType::THREE_OF_A_KIND;
		} else if is_two_pair(frequency, jokers) {
			return HType::TWO_PAIR;
		} else if frequency.contains(&(2-jokers)) {
			return HType::ONE_PAIR;
		} else {
			return HType::HIGH_CARD;
		}
		fn is_full_house(mut frequency: [usize; 14], jokers: usize) -> bool {
			if frequency.contains(&(3-jokers)) {
				let i: usize = frequency.iter().position(|i| i == &(3-jokers)).unwrap();
				frequency[i] = 0;
				return frequency.contains(&2);
			}
			return false;
		}

		fn is_two_pair(mut frequency: [usize; 14], jokers: usize) -> bool {
			if frequency.contains(&(2-jokers)) {
				let i: usize = frequency.iter().position(|i| i == &(2-jokers)).unwrap();
				frequency[i] = 0;
				return frequency.contains(&2);
			}
			return false;
		}
	}

	pub fn new(name: String) -> Self {
		let cards: [usize; 5] = Self::name_to_cards(&name);
		
		let htype: HType = Self::cards_to_htype(cards);
		return Self {
			name: name,
			cards: cards,
			htype: htype,
		};
	}
}
impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.partial_cmp(other).unwrap();
	}
}
impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.htype != other.htype {
			return Some(self.htype.cmp(&other.htype));
		} else {
			for (i, j) in self.cards.iter().zip(other.cards.iter()) {
				if i != j {
					return Some(i.cmp(j));
				}
			} return Some(Ordering::Equal);
		}
	}
}
impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		return self.cards == other.cards;
	}
}

#[derive(Debug, Eq)]
pub struct HType {
	name: &'static str, 
	value: usize,
}
impl HType {
	const FIVE_OF_A_KIND: 	Self 	= Self{name: "Five of a Kind", 	value: 6};
	const FOUR_OF_A_KIND: 	Self 	= Self{name: "Four of a Kind", 	value: 5};
	const FULL_HOUSE: 		Self 	= Self{name: "Full House", 			value: 4};
	const THREE_OF_A_KIND: 	Self 	= Self{name: "Three of a Kind", 	value: 3};
	const TWO_PAIR: 		Self 	= Self{name: "Two Pair", 			value: 2};
	const ONE_PAIR: 		Self 	= Self{name: "One Pair", 			value: 1};
	const HIGH_CARD: 		Self	= Self{name: "High Card", 			value: 0};
}
impl Ord for HType {
	fn cmp(&self, other: &Self) -> Ordering {
		return self.partial_cmp(other).unwrap();
	}
}
impl PartialOrd for HType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return Some(self.value.cmp(&other.value));
	}
}
impl PartialEq for HType {
	fn eq(&self, other: &Self) -> bool {
		return self.value == other.value;
	}
}

fn pt_0() -> usize{
	let mut sets: Vec<Set> = Vec::new();
	for line in fs::read_to_string(INPUT_FILE).unwrap().lines() {
		let line: Vec<&str> = line.split(" ").collect();
		let name: String = String::from(line[0]);
		let bid: usize = line[1].parse().unwrap();
		
		let set = Set::new(name, bid);
		sets.push(set);
	}
	
	sets.sort();
	let mut sum: usize = 0;
	for (i, set) in sets.into_iter().enumerate() {
		sum += set.bid*(i+1);
	}
	return sum;
}
fn pt_1() -> usize {
	WITH_JOKERS.store(false, atomic::Ordering::Relaxed);
	return pt_0();
}

fn pt_2() -> usize {
	WITH_JOKERS.store(true, atomic::Ordering::Relaxed);
	return pt_0();
}
