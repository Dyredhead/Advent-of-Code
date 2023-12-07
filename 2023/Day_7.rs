/*
Step 1: define 'hand' struct, which contains the hand
Step 2: implement a compare function which when given two hands returns a type Ordering
Step 1: write a compare function, that given two hands, 
*/

use std::cmp::Ordering;

fn main() {
	return 0;
}

fn hand_str_to_array(hand: &str) {
	let map_card_to_value: HashMap<(char, usize)> = HashMap::from([ 
		('A', 14),
		('K', 13),
		('Q', 12),
		('J', 11),
		('T', 10),
	]);
	
	let mut hand_new: [usize; 5] = [0; 5];
	for (i, card) in hand.enumerate() {
		if card.is_ascii_digit() {
			hand_new[i] = card.to_digit(10).unwrap();
		} else {
			hand[i] = map_card_to_value.get(card).unwrap();
		}
	}
}


enum Type {
	FIVE_OF_A_KIND,
	FOUR_OF_A_KIND,
	FULL_HOUSE,
	THREE_OF_A_KIND,
	TWO_PAIR,
	ONE_PAIR,
	HIGH_CARD,
}

impl Ord for Type {
	let map: HashMap<(Type, usize)> = HashMap::from([ 
		(Type::FIVE_OF_A_KIND, 6),
		(Type::FOUR_OF_A_KIND, 5),
		(Type::FULL_HOUSE, 4),
		(Type::THREE_OF_A_KIND, 3),
		(Type::TWO_PAIR, 2),
		(Type::ONE_PAIR, 1),
		(Type::HIGH_CARD, 0),
	]);

	fn cmp(&self, other: &self) -> Ordering {
		return map.get(self).cmp(map.get(other));
	}
}

pub struct Set {
	hand: [usize; 5],
	bid: usize,
}

impl Ord for Set {
	fn cmp(&self, other: &self) -> Ordering {
		if self.hand.type().cmp(other.hand.type()) != Ordering::Equal {
			return self.hand.type().cmp(other.hand.type());
		} else {
			for (i, j) in self.hand.into_iter().zip(other.hand.into_iter()) {
				if i != j {
					return i.cmp(j);
				}
			} return Ordering:Equal;
		}
	}
}

let sets: Vec<(&str, usize)> = Vec::new();
sets.sort_by(|a, b| cmp(a, b));
let mut sum: usize = 0;
for (i, set) in sets.enumerate() {
	sum += set.bid*(i+1);
}
return sum;

