#![feature(new_range_api)]
#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use std::{
    cmp::{max, min},
    collections::{HashSet, hash_set},
    iter::Empty,
    mem::swap,
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
    let mut ranges: HashSet<_> = input
        .0
        .lines()
        .map(|range| {
            let range = range.split_once('-').unwrap();
            let start: usize = range.0.parse().unwrap();
            let last: usize = range.1.parse().unwrap();
            RangeInclusive { start, last }
        })
        .collect();

    // let mut final_ranges: HashSet<RangeInclusive<usize>> = HashSet::new();
    loop {
        let mut new_ranges: HashSet<RangeInclusive<usize>> = HashSet::new();
        for (i, a) in ranges.iter().enumerate() {
            // let mut flag_disjoint_from_all = true;
            // let mut flag_always_a_subset = true;
            for (j, b) in ranges.iter().enumerate() {
                if i == j {
                    continue;
                }

                let intersection = a.intersect(*b);
                if !intersection.is_empty() {
                    let start = min(a.start, b.start);
                    let last = max(a.last, b.last);

                    if let Bound::Included(c) = intersection.0
                        && let Bound::Included(d) = intersection.1
                    {
                        let left = RangeInclusive { start, last: c };
                        if !left.is_empty() {
                            new_ranges.insert(left);
                        }

                        let middle = RangeInclusive {
                            start: c + 1,
                            last: d - 1,
                        };
                        if !middle.is_empty() {
                            new_ranges.insert(middle);
                        }

                        let right = RangeInclusive { start: d, last };
                        if !right.is_empty() {
                            new_ranges.insert(right);
                        }
                    }
                }

                //     let diff = range_diffrence(*b, *a);
                //     match diff {
                //         SetDiffrence::Empty => {
                //             flag_disjoint_from_all = false;
                //             flag_always_a_subset = false;
                //         }
                //         SetDiffrence::CompletelyDisjoint(_) => {
                //             flag_always_a_subset = false;
                //         }
                //         SetDiffrence::PartiallyDisjoint(lhs, rhs) => {
                //             new_ranges.insert(lhs);
                //             new_ranges.insert(rhs);
                //             flag_disjoint_from_all = false;
                //             flag_always_a_subset = false;
                //         }
                //         SetDiffrence::Subset(lhs, rhs) => {
                //             if let Some(lhs) = lhs {
                //                 new_ranges.insert(lhs);
                //             }
                //             if let Some(rhs) = rhs {
                //                 new_ranges.insert(rhs);
                //             }
                //             flag_disjoint_from_all = false;
                //         }
                //     }
                // }
                // if flag_disjoint_from_all || flag_always_a_subset {
                //     final_ranges.insert(*a);
                // }
            }

            dbg!(&ranges);
            // if new_ranges.is_empty() {
            //     break;
            // } else {
            // }
        }
        ranges = new_ranges;
    }
    dbg!(&ranges);
    let count = ranges.iter().fold(0, |acc, range| acc + len(range));
    return Some(u64::try_from(count).unwrap());
}

#[derive(Eq, PartialEq)]
enum SetDiffrence {
    Empty,
    PartiallyDisjoint(RangeInclusive<usize>, RangeInclusive<usize>),
    CompletelyDisjoint(RangeInclusive<usize>),
    Subset(Option<RangeInclusive<usize>>, Option<RangeInclusive<usize>>),
}

fn range_diffrence(lhs: RangeInclusive<usize>, rhs: RangeInclusive<usize>) -> SetDiffrence {
    if lhs == rhs {
        return SetDiffrence::Empty;
    }

    if rhs.start <= lhs.start && lhs.last <= rhs.last {
        // swap(&mut lhs, &mut rhs);
        return SetDiffrence::Empty;
    }

    if lhs.intersect(rhs).is_empty() {
        return SetDiffrence::CompletelyDisjoint(lhs);
    } else if lhs.start <= rhs.start && rhs.last <= lhs.last {
        let left = RangeInclusive {
            start: lhs.start,
            last: rhs.start - 1,
        };

        let right = RangeInclusive {
            start: rhs.last + 1,
            last: lhs.last,
        };

        return SetDiffrence::Subset(
            if !left.is_empty() { Some(left) } else { None },
            if !right.is_empty() { Some(right) } else { None },
        );
    } else {
        return SetDiffrence::PartiallyDisjoint(
            RangeInclusive {
                start: min(lhs.start, rhs.start),
                last: max(lhs.start, rhs.start),
            },
            RangeInclusive {
                start: min(lhs.last, rhs.last),
                last: max(lhs.last, rhs.last),
            },
        );
    }

    // else if rhs.start <= lhs.start && rhs.last <= lhs.last {
    //     return (SetDiffrence::PartiallyDisjoint(
    //         RangeInclusive {
    //             start: rhs.start,
    //             last: lhs.start,
    //         },
    //         RangeInclusive {
    //             start: rhs.last,
    //             last: lhs.last,
    //         },
    //     ));
    // } else if lhs.start <= rhs.start && lhs.last <= rhs.last {
    //     return SetDiffrence::PartiallyDisjoint(RangeInclusive {
    //         start: lhs.start,
    //         last: rhs.start,
    //     });
    // } else {
    //     unreachable!()
    // }
}

fn len(range: &RangeInclusive<usize>) -> usize {
    assert!(range.last >= range.start);
    if range.is_empty() {
        return 0;
    } else {
        return range.last - range.start + 1;
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
