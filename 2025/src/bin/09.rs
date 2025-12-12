#![feature(new_range_api)]

advent_of_code::solution!(9);

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    range::{Range, RangeInclusive},
};

use itertools::Itertools;
use strum_macros::VariantArray;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Location {
    Inside,
    Border,
    Outside,
}

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

    // get bounding box of polygon
    let (min_x, max_x) = match input.iter().minmax_by(|a, b| a.x.cmp(&b.x)) {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        // add one to max_x.x so that if there is no edge against bounding box
        itertools::MinMaxResult::MinMax(min_x, max_x) => (min_x.x, max_x.x + 1),
    };
    dbg!(min_x, max_x);

    let (min_y, max_y) = match input.iter().minmax_by(|a, b| a.y.cmp(&b.y)) {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        itertools::MinMaxResult::MinMax(min_y, max_y) => (min_y.y, max_y.y),
    };
    dbg!(min_y, max_y);

    // find all borders of simple polygon
    let mut edge_points: HashSet<Point> = HashSet::new();
    for window in input.windows(2) {
        let (a, b) = (window[0], window[1]);
        if a.x == b.x {
            for y in min(a.y, b.y)..=max(a.y, b.y) {
                edge_points.insert(Point::new(a.x, y));
            }
        } else {
            for x in min(a.x, b.x)..=max(a.x, b.x) {
                edge_points.insert(Point::new(x, a.y));
            }
        };
    }

    let mut edge_points = Vec::from_iter(edge_points);
    edge_points.sort();
    // dbg!(&edge_points);

    let mut map: HashMap<usize, Vec<Range<usize>>> = HashMap::with_capacity(max_y - min_y);
    let mut dp: HashMap<Point, bool> = HashMap::with_capacity(edge_points.len());
    // find all inside ranges by scaning right to left
    for point in edge_points.iter().rev() {
        let next_point = Point::new(point.x + 1, point.y);

        if edge_points.contains(&next_point) {
            let previous_point = Point::new(point.x - 1, point.y);
            if edge_points.contains(&previous_point) {
                dp.insert(*point, *dp.get(&next_point).unwrap());
            } else {
                dp.insert(*point, !dp.get(&next_point).unwrap());
            }

            continue;
        }
        dbg!(point);
        let start_x = point.x;
        let mut end_x: Option<usize> = None;
        // skip start_x since its on the edge
        for x in (start_x..=max_x).skip(1) {
            let point = Point::new(x, point.y);
            if edge_points.contains(&point) {
                end_x = Some(x);
                break;
            }
        }
        dbg!(end_x);

        match end_x {
            None => {
                dp.insert(*point, false);
            }
            Some(end_x) => {
                // SAFTEY: since we are looping from the end,
                // if end_x is another point, then it must be in is_inside already
                let end_point = Point::new(end_x, point.y);
                let is_start_point_inside = !dp.get(&end_point).unwrap();
                dp.insert(*point, !is_start_point_inside);
                if is_start_point_inside {
                    let range = Range {
                        start: start_x,
                        end: end_x,
                    };
                    match map.get_mut(&point.y) {
                        Some(ranges) => {
                            ranges.push(range);
                        }
                        None => {
                            let ranges = Vec::from_iter([range]);
                            map.insert(point.y, ranges);
                        }
                    }
                }
            }
        }
    }

    dbg!(map.iter().sorted_by_key(|x| x.0));

    // dbg!();

    // // find inner point using marching ray
    // let (a, b) = (input[0], input[1]);
    // let (edge_point, direction) = if a.x == b.x {
    //     let edge_point = Point::new(a.x, min(a.y, b.y) + 1);

    //     // go towards closer bound to save time
    //     let direction = if edge_point.y - 0 < max_y - edge_point.y {
    //         Direction::Left
    //     } else {
    //         Direction::Right
    //     };

    //     (edge_point, direction)
    // } else {
    //     let edge_point = Point::new(min(a.x, b.x) + 1, a.y);

    //     // go towards closer bound to save time
    //     let direction = if edge_point.x - 0 < max_x - edge_point.x {
    //         Direction::Up
    //     } else {
    //         Direction::Down
    //     };

    //     (edge_point, direction)
    // };

    // dbg!();

    // let mut intersections = 0;
    // let mut marching_ray = edge_point;
    // while 0 < marching_ray.x
    //     && marching_ray.x < max_x
    //     && 0 < marching_ray.y
    //     && marching_ray.y < max_y
    // {
    //     marching_ray = marching_ray.shift(&direction);

    //     if points.contains(&marching_ray) {
    //         intersections += 1;
    //     }
    // }

    // let direction = if intersections % 2 == 0 {
    //     direction.reverse()
    // } else {
    //     direction
    // };
    // let inside_point = edge_point.shift(&direction);

    // dbg!();

    // // find all points inside simple polygon using Flood Fill
    // let mut to_visit = VecDeque::with_capacity(capacity / 100);
    // to_visit.push_back(inside_point);
    // // let mut to_visit = VecDeque::from_iter([inside_point]);
    // points.insert(inside_point);
    // let mut n = 0;
    // while !to_visit.is_empty() {
    //     n += 1;
    //     let point = to_visit.pop_front().unwrap();
    //     for direction in Direction::VARIANTS {
    //         let new_point = point.shift(direction);
    //         if !points.contains(&new_point) {
    //             to_visit.push_back(new_point);
    //             points.insert(new_point);
    //         }
    //     }
    //     if n % 1_000_000 == 0 {
    //         dbg!(to_visit.len());
    //     }
    // }

    // dbg!();

    // TODO: possible that there could be an entierly enclosed space where the borders touch each other. Might have to scale everything up by 3x to avoid such a possibility
    //
    // for each unqiue pair of red squares:
    //      march 4 dots on each side of the perimiter
    //      if any dot crosses a point not in the polygon skip to next pair of squares
    //      else save area if maximum;
    // let mut max_area = 0;
    // for (i, a) in input.iter().enumerate() {
    //     'pairs: for b in input.iter().skip(i + 1) {
    //         let x_range = RangeInclusive::new(min(a.x, b.x), max(a.x, b.x));
    //         let y_range = RangeInclusive::new(min(a.y, b.y), max(a.y, b.y));

    //         for x in x_range {
    //             let top_point = Point::new(x, max(a.y, b.y));
    //             let bottom_point = Point::new(x, min(a.y, b.y));
    //             if !(points.contains(&top_point) && points.contains(&bottom_point)) {
    //                 continue 'pairs;
    //             }
    //         }

    //         for y in y_range {
    //             let right_point = Point::new(max(a.x, b.x), y);
    //             let left_point = Point::new(min(a.x, b.x), y);
    //             if !(points.contains(&right_point) && points.contains(&left_point)) {
    //                 continue 'pairs;
    //             }
    //         }

    //         let delta_x = max(a.x, b.x) - min(a.x, b.x) + 1;
    //         let delta_y = max(a.y, b.y) - min(a.y, b.y) + 1;
    //         let area = delta_x * delta_y;
    //         dbg!(a, b, area);
    //         max_area = max(max_area, area);
    //     }
    // }

    // dbg!();

    // return Some(max_area);
    None
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
