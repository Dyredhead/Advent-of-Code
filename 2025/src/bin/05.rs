#![feature(new_range_api)]
#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use std::{
    ops::{Bound, IntoBounds, RangeBounds},
    range::RangeInclusive,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = input
        .0
        .lines()
        .map(|range| {
            let range = range.split_once('-').unwrap();
            let start: usize = range.0.parse().unwrap();
            let last: usize = range.1.parse().unwrap();
            RangeInclusive { start, last }
        })
        .collect();

    let ids: Vec<usize> = input.1.lines().map(|id| id.parse().unwrap()).collect();

    let mut count = 0;
    for id in ids {
        for range in ranges.iter() {
            if range.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    return Some(count);
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = input
        .0
        .lines()
        .map(|range| {
            let range = range.split_once('-').unwrap();
            let start: usize = range.0.parse().unwrap();
            let last: usize = range.1.parse().unwrap();
            RangeInclusive { start, last }
        })
        .collect();

    let mut count: isize = 0;
    for (index, range1) in ranges.iter().enumerate() {
        count += len(range1) as isize;
        for range2 in ranges.iter().skip(index + 1) {
            count -= len_intersection(range1, range2) as isize;
        }
    }
    return Some(u64::try_from(count).unwrap());
}

fn len(range: &RangeInclusive<usize>) -> usize {
    assert!(range.last >= range.start);
    if range.is_empty() {
        return 0;
    } else {
        return range.last - range.start + 1;
    }
}

fn len_intersection(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> usize {
    let intersection = a.intersect(*b);

    if intersection.is_empty() {
        return 0;
    } else {
        if let Bound::Included(start) = intersection.0
            && let Bound::Included(last) = intersection.1
        {
            assert!(last >= start);
            return last - start + 1;
        } else {
            unreachable!()
        }
    }
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
        dbg!(result);
        // assert_eq!(result, Some(14));
    }
}
