advent_of_code::solution!(9);

use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

use itertools::Itertools;
use strum::VariantArray;
use strum_macros::VariantArray;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Location {
    Inside,
    Border,
    Outside,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    // location: Option<Location>,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        return Self {
            x,
            y,
            // location: None,
        };
    }

    // fn with_location(x: usize, y: usize, location: Location) -> Self {
    //     return Self {
    //         x,
    //         y,
    //         location: Some(location),
    //     };
    // }
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

#[derive(Debug, VariantArray, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Point {
    fn shift(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
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
        itertools::MinMaxResult::MinMax(min_x, max_x) => (min_x.x, max_x.x),
    };
    dbg!(min_x, max_x);

    let (min_y, max_y) = match input.iter().minmax_by(|a, b| a.y.cmp(&b.y)) {
        itertools::MinMaxResult::NoElements => unreachable!(),
        itertools::MinMaxResult::OneElement(_) => unreachable!(),
        itertools::MinMaxResult::MinMax(min_y, max_y) => (min_y.y, max_y.y),
    };
    dbg!(min_y, max_y);

    let capacity = (max_x - min_x) * (max_y - min_y) / 100;

    // find all borders of simple polygon
    let mut points: HashSet<Point> = HashSet::with_capacity(capacity);
    for window in input.windows(2) {
        let (a, b) = (window[0], window[1]);
        if a.x == b.x {
            let y_range = RangeInclusive::new(min(a.y, b.y), max(a.y, b.y));
            for y in y_range {
                points.insert(Point::new(a.x, y));
            }
        } else {
            let x_range = RangeInclusive::new(min(a.x, b.x), max(a.x, b.x));
            for x in x_range {
                points.insert(Point::new(x, a.y));
            }
        };
    }
    dbg!();

    // find inner point using marching ray
    let (a, b) = (input[0], input[1]);
    let (edge_point, direction) = if a.x == b.x {
        let edge_point = Point::new(a.x, min(a.y, b.y) + 1);

        // go towards closer bound to save time
        let direction = if edge_point.y - 0 < max_y - edge_point.y {
            Direction::Left
        } else {
            Direction::Right
        };

        (edge_point, direction)
    } else {
        let edge_point = Point::new(min(a.x, b.x) + 1, a.y);

        // go towards closer bound to save time
        let direction = if edge_point.x - 0 < max_x - edge_point.x {
            Direction::Up
        } else {
            Direction::Down
        };

        (edge_point, direction)
    };

    dbg!();

    let mut intersections = 0;
    let mut marching_ray = edge_point;
    while 0 < marching_ray.x
        && marching_ray.x < max_x
        && 0 < marching_ray.y
        && marching_ray.y < max_y
    {
        marching_ray = marching_ray.shift(&direction);

        if points.contains(&marching_ray) {
            intersections += 1;
        }
    }

    let direction = if intersections % 2 == 0 {
        direction.reverse()
    } else {
        direction
    };
    let inside_point = edge_point.shift(&direction);

    dbg!();

    // find all points inside simple polygon using Flood Fill
    let mut to_visit = VecDeque::with_capacity(capacity / 100);
    to_visit.push_back(inside_point);
    // let mut to_visit = VecDeque::from_iter([inside_point]);
    points.insert(inside_point);
    let mut n = 0;
    while !to_visit.is_empty() {
        n += 1;
        let point = to_visit.pop_front().unwrap();
        for direction in Direction::VARIANTS {
            let new_point = point.shift(direction);
            if !points.contains(&new_point) {
                to_visit.push_back(new_point);
                points.insert(new_point);
            }
        }
        if n % 1_000_000 == 0 {
            dbg!(to_visit.len());
        }
    }

    dbg!();

    // TODO: possible that there could be an entierly enclosed space where the borders touch each other. Might have to scale everything up by 3x to avoid such a possibility
    //
    // for each unqiue pair of red squares:
    //      march 4 dots on each side of the perimiter
    //      if any dot crosses a point not in the polygon skip to next pair of squares
    //      else save area if maximum;
    let mut max_area = 0;
    for (i, a) in input.iter().enumerate() {
        'pairs: for b in input.iter().skip(i + 1) {
            let x_range = RangeInclusive::new(min(a.x, b.x), max(a.x, b.x));
            let y_range = RangeInclusive::new(min(a.y, b.y), max(a.y, b.y));

            for x in x_range {
                let top_point = Point::new(x, max(a.y, b.y));
                let bottom_point = Point::new(x, min(a.y, b.y));
                if !(points.contains(&top_point) && points.contains(&bottom_point)) {
                    continue 'pairs;
                }
            }

            for y in y_range {
                let right_point = Point::new(max(a.x, b.x), y);
                let left_point = Point::new(min(a.x, b.x), y);
                if !(points.contains(&right_point) && points.contains(&left_point)) {
                    continue 'pairs;
                }
            }

            let delta_x = max(a.x, b.x) - min(a.x, b.x) + 1;
            let delta_y = max(a.y, b.y) - min(a.y, b.y) + 1;
            let area = delta_x * delta_y;
            max_area = max(max_area, area);
        }
    }

    dbg!();

    return Some(max_area);
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
