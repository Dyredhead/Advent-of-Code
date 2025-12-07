#![feature(new_range_api)]
#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use std::{
    collections::BTreeSet,
    fmt::Debug,
    ops::{Bound, Deref, IntoBounds, RangeBounds},
    range::RangeInclusive,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = input
        .0
        .lines()
        .map(|range| {
            let range = range.split_once('-').unwrap();
            let start: usize = range.0.parse().unwrap();
            let last: usize = range.1.parse().unwrap();
            CustomRangeInclusive(RangeInclusive { start, last })
        })
        .collect();

    let ids: Vec<usize> = input.1.lines().map(|id| id.parse().unwrap()).collect();

    let mut count = 0;
    for id in ids {
        for range in ranges.iter() {
            if range.0.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    return Some(count);
}

#[derive(Clone, Copy)]
struct CustomRangeInclusive(RangeInclusive<usize>);

impl Debug for CustomRangeInclusive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl CustomRangeInclusive {
    fn new(start: usize, last: usize) -> Self {
        CustomRangeInclusive(RangeInclusive { start, last })
    }

    fn new_empty() -> Self {
        CustomRangeInclusive(RangeInclusive { start: 1, last: 0 })
    }
}

impl PartialOrd for CustomRangeInclusive {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.start.partial_cmp(&other.0.start)
    }
}

impl Ord for CustomRangeInclusive {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.start.cmp(&other.0.start)
    }
}

impl PartialEq for CustomRangeInclusive {
    fn eq(&self, other: &Self) -> bool {
        self.0.start == other.0.start
    }
}

impl Eq for CustomRangeInclusive {}

impl Deref for CustomRangeInclusive {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = input
        .0
        .lines()
        .map(|range| {
            let range = range.split_once('-').unwrap();
            let start: usize = range.0.parse().unwrap();
            let last: usize = range.1.parse().unwrap();
            CustomRangeInclusive::new(start, last)
        })
        .collect();

    let mut disjoint_ranges: BTreeSet<CustomRangeInclusive> = BTreeSet::new();
    for mut range in ranges {
        let mut to_insert: Vec<CustomRangeInclusive> = Vec::new();

        for &disjoint_range in disjoint_ranges.iter() {
            let intersection = disjoint_range.intersect(*range);
            // if range is completly disjoint from the disjoint range
            if intersection.is_empty() {
                continue;
            }

            // if disjoint range completly contains range
            // set the range to be empty, so it doesnt get added to the to_insert list
            if disjoint_range.start <= range.start && range.last <= disjoint_range.last {
                range = CustomRangeInclusive::new_empty();
                break;
            }

            let left = CustomRangeInclusive::new(range.start, disjoint_range.start - 1);
            let right = CustomRangeInclusive::new(disjoint_range.last + 1, range.last);

            if !left.is_empty() {
                to_insert.push(left);
            }
            range = right;
        }
        if !range.is_empty() {
            to_insert.push(range);
        }

        for range in to_insert {
            disjoint_ranges.insert(range);
        }
    }

    let sum = disjoint_ranges
        .iter()
        .fold(0, |sum, range| sum + range.last - range.start + 1);

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
