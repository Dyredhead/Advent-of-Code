advent_of_code::solution!(5);
// Imports
use std::cmp::{max, min};
use std::ops::Range;

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();

    let mut seeds: Vec<usize> = lines[0]["seeds: ".len()..]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps: Vec<Vec<Vec<usize>>> = Vec::new();

    let mut i: usize = 0;
    let mut line_iter = lines.into_iter().skip(1);

    while let Some(next) = line_iter.next() {
        if next == "" {
            i += 1;
            line_iter.next();
            maps.push(Vec::new());
        } else {
            let map = next.split(" ").map(|x| x.parse().unwrap()).collect();
            maps[i - 1].push(map);
        }
    }

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

    return Some(*seeds.iter().min().unwrap());
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();

    let range_seeds: Vec<Range<usize>> = lines[0]["seeds: ".len()..]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<Range<usize>>>();

    let mut maps: Vec<Vec<[Range<usize>; 2]>> = Vec::new();

    let mut i = 0;
    let mut line_iter = lines.into_iter().skip(1);

    while let Some(next) = line_iter.next() {
        if next == "" {
            i += 1;
            line_iter.next();
            maps.push(Vec::<[std::ops::Range<usize>; 2]>::new());
        } else {
            let map: Vec<usize> = next.split(" ").map(|x| x.parse().unwrap()).collect();

            let map: [std::ops::Range<usize>; 2] =
                [(map[0]..map[0] + map[2]), (map[1]..map[1] + map[2])];
            maps[i - 1].push(map);
        }
    }

    fn ranges(seed: &Range<usize>, source: &Range<usize>) -> [Range<usize>; 3] {
        return [
            Range {
                start: seed.start,
                end: source.start,
            },
            Range {
                start: max(seed.start, source.start),
                end: min(seed.end, source.end),
            },
            Range {
                start: source.end,
                end: seed.end,
            },
        ];
    }

    let mut range_seeds_old: Vec<Range<usize>> = range_seeds;
    for map_type in maps {
        let mut range_seeds_new: Vec<Range<usize>> = Vec::new();
        for map in &map_type {
            let mut range_seeds_next: Vec<Range<usize>> = Vec::new();
            for range_seed in &range_seeds_old {
                let range_destination: &Range<usize> = &map[0];
                let range_source: &Range<usize> = &map[1];

                let range_seed_old: &Range<usize> = &range_seed;
                let [range_left, range_overlap, range_right] = ranges(range_seed_old, range_source);
                if !range_overlap.is_empty() {
                    // offset needed because the overlap may start in the center of the source range, rather than at the start.
                    let offset: usize = range_overlap.start - range_source.start;
                    let size: usize = range_overlap.end - range_overlap.start;
                    let range_seed_new: Range<usize> = Range {
                        start: range_destination.start + offset,
                        end: range_destination.start + offset + size,
                    };
                    range_seeds_new.push(range_seed_new);

                    if !range_left.is_empty() {
                        range_seeds_next.push(range_left.clone());
                    }
                    if !range_right.is_empty() {
                        range_seeds_next.push(range_right.clone());
                    }
                } else {
                    range_seeds_next.push(range_seed_old.clone());
                }
            }
            range_seeds_old = range_seeds_next.clone();
        }
        // needed to account for any seeds not being mapped, thus carrying over with the same value as before
        range_seeds_new.append(&mut range_seeds_old);
        range_seeds_old = range_seeds_new.clone();
    }
    let range_seeds = range_seeds_old;
    let min = range_seeds
        .iter()
        .fold(range_seeds[0].start, |_min, i| min(_min, i.start));
    return Some(min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
