#![feature(new_range_api)]
#![feature(negative_impls)]

advent_of_code::solution!(9);

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops, range,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        return Self { x, y };
    }
}

fn contains(ranges: &Vec<range::RangeInclusive<usize>>, point: Point) -> bool {
    for range in ranges {
        if range.contains(&point.x) {
            return true;
        }
    }
    return false;
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<(usize, usize)> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut max_area = 0;
    for (i, a) in input.iter().enumerate() {
        for b in input.iter().skip(i + 1) {
            if !(a.0 == b.0 || a.1 == b.1) {
                let delta_x = max(a.0, b.0) - min(a.0, b.0) + 1;
                let delta_y = max(a.1, b.1) - min(a.1, b.1) + 1;
                let area = delta_x * delta_y;
                max_area = max(max_area, area);
            }
        }
    }

    return Some(max_area);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input: Vec<Point> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    input.push(*input.first().unwrap());

    // find all borders of simple polygon
    let mut vertical_edge_points: HashSet<Point> = HashSet::new();
    let mut horizontal_edge_points: HashMap<Point, Point> = HashMap::new();
    let mut horizontal_edge_ranges: HashMap<usize, Vec<range::RangeInclusive<usize>>> =
        HashMap::new();
    for window in input.windows(2) {
        let (a, b) = (window[0], window[1]);
        // Vertical
        if a.x == b.x {
            for y in min(a.y, b.y)..=max(a.y, b.y) {
                vertical_edge_points.insert(Point::new(a.x, y));
            }
        }
        // Horizontal
        else {
            horizontal_edge_points.insert(
                Point::new(min(a.x, b.x), a.y),
                Point::new(max(a.x, b.x), a.y),
            );
            let range = range::RangeInclusive {
                start: min(a.x, b.x),
                last: max(a.x, b.x),
            };
            match horizontal_edge_ranges.get_mut(&a.y) {
                Some(ranges) => {
                    ranges.push(range);
                }
                None => {
                    horizontal_edge_ranges.insert(a.y, Vec::from_iter([range]));
                }
            }
        }
    }

    let sorted_edge_points = {
        let mut edge_points = Vec::from_iter(vertical_edge_points.iter());
        edge_points.sort();
        edge_points
    };

    let mut map: HashMap<usize, Vec<range::RangeInclusive<usize>>> = HashMap::new();
    let mut dp: HashMap<Point, bool> = HashMap::with_capacity(sorted_edge_points.len());
    // find all inside ranges by scaning right to left
    for point in sorted_edge_points.into_iter().rev() {
        let start_x = point.x;

        // vecs will awlays either be None or len >= 1
        let end_x = match map.get(&point.y) {
            Some(ranges) => Some(ranges.last().unwrap().start),
            None => None,
        };

        match end_x {
            None => {
                dp.insert(*point, false);
                let range = range::RangeInclusive {
                    start: start_x,
                    last: start_x,
                };
                map.insert(point.y, Vec::from_iter([range]));
            }
            Some(end_x) => {
                let mut end_point = Point::new(end_x, point.y);
                let is_start_point_inside = match horizontal_edge_points.get(&end_point) {
                    Some(point) => {
                        end_point = *point;
                        dp.get(&end_point).unwrap().to_owned()
                    }
                    // Hit a vertical edge
                    None => !dp.get(&end_point).unwrap().to_owned(),
                };

                dp.insert(*point, is_start_point_inside);
                let ranges = map.get_mut(&point.y).unwrap();
                if is_start_point_inside {
                    let range = range::RangeInclusive {
                        start: start_x,
                        last: end_x,
                    };
                    *ranges.last_mut().unwrap() = range;
                } else {
                    let range = range::RangeInclusive {
                        start: start_x,
                        last: start_x,
                    };
                    ranges.push(range);
                };
            }
        }
    }

    // dbg!(map.iter().sorted_by_key(|(key, _)| **key));
    // dbg!(
    //     horizontal_edge_ranges
    //         .iter()
    //         .sorted_by_key(|(key, _)| **key)
    // );

    for (y, x_ranges) in horizontal_edge_ranges {
        let last = map.get_mut(&y).unwrap().last_mut().unwrap();
        let first = x_ranges
            .iter()
            .min_by(|a, b| a.start.cmp(&b.start))
            .unwrap();
        if last != first {
            *last = *first;
        }
    }

    // let total = map.iter().fold(0, |acc, (_, ranges)| acc + ranges.len());
    // dbg!(total);

    // dbg!(map.iter().sorted_by_key(|(key, _)| **key));

    // TODO: possible that there could be an entierly enclosed space where the borders touch each other. Might have to scale everything up by 3x to avoid such a possibility

    // for each unqiue pair of red squares:
    //      march 4 dots on each side of the perimiter
    //      if any dot crosses a point not in the polygon skip to next pair of squares
    //      else save area if maximum;

    let mut max_area = 0;
    for (i, a) in input.iter().enumerate() {
        'pairs: for b in input.iter().skip(i + 1) {
            let x_range = ops::RangeInclusive::new(min(a.x, b.x), max(a.x, b.x));
            let top_ranges = map.get(&max(a.y, b.y)).unwrap();
            let bottom_ranges = map.get(&min(a.y, b.y)).unwrap();

            for x in x_range {
                let top_point = Point::new(x, max(a.y, b.y));
                let bottom_point = Point::new(x, min(a.y, b.y));

                if !(contains(top_ranges, top_point) && contains(bottom_ranges, bottom_point)) {
                    continue 'pairs;
                }
            }

            let y_range = ops::RangeInclusive::new(min(a.y, b.y), max(a.y, b.y));
            for y in y_range {
                let ranges = map.get(&y).unwrap();
                let right_point = Point::new(max(a.x, b.x), y);
                let left_point = Point::new(min(a.x, b.x), y);
                if !(contains(ranges, right_point) && contains(ranges, left_point)) {
                    continue 'pairs;
                }
            }

            let delta_x = max(a.x, b.x) - min(a.x, b.x) + 1;
            let delta_y = max(a.y, b.y) - min(a.y, b.y) + 1;
            let area = delta_x * delta_y;
            // dbg!(a, b, area);
            max_area = max(max_area, area);
        }
    }

    return Some(max_area);
    // return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
