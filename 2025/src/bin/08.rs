advent_of_code::solution!(8);

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    vec,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl Debug for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

type Circuit = Vec<JunctionBox>;

fn distance(a: JunctionBox, b: JunctionBox) -> f32 {
    let delta_x = max(a.x, b.x) - min(a.x, b.x);
    let delta_y = max(a.y, b.y) - min(a.y, b.y);
    let delta_z = max(a.z, b.z) - min(a.z, b.z);
    return f32::sqrt((delta_x.pow(2) + delta_y.pow(2) + delta_z.pow(2)) as f32);
}

pub fn part_one(input: &str) -> Option<usize> {
    let junction_boxes: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            let mut temp = line.split(',');
            JunctionBox {
                x: temp.next().unwrap().parse().unwrap(),
                y: temp.next().unwrap().parse().unwrap(),
                z: temp.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut circuits: HashSet<Circuit> = junction_boxes
        .iter()
        .map(|&junction_box| Circuit::from([junction_box]))
        .collect();

    // println!("circuits: {:?}\n", &circuits);

    let mut map: HashMap<JunctionBox, Circuit> = junction_boxes
        .iter()
        .map(|junction_box| (*junction_box, Circuit::from([*junction_box])))
        .collect();

    // println!("map: {:?}\n", &map);

    let mut pairs: Vec<(JunctionBox, JunctionBox, f32)> =
        Vec::with_capacity(junction_boxes.len().pow(2) / 2);

    for (i, &a) in junction_boxes.iter().enumerate() {
        for &b in junction_boxes.iter().skip(i + 1) {
            pairs.push((a, b, distance(a, b)));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    // println!("pairs: {:?}\n", &pairs);
    let mut pairs = pairs.into_iter();

    let mut counter = 0;
    while counter < 1000 - 1 {
        let pair = pairs.next().unwrap();

        if map.get(&pair.0).unwrap().contains(&pair.1) {
            continue;
        }

        let a = pair.0;
        let b = pair.1;

        let circuit_a = map.get(&a).unwrap().clone();
        let circuit_b = map.get(&b).unwrap().clone();

        // println!("Before");
        // println!("circuit_a: {:?}", &circuit_a);
        // println!("circuit_b: {:?}", &circuit_b);

        let new_circuit: Circuit = circuit_a
            .iter()
            .chain(circuit_b.iter())
            .map(|j| *j)
            .collect();

        // println!("After:");
        // println!("circuit_a: {:?}", &circuit_a);
        // println!("circuit_b: {:?}", &circuit_b);
        // println!("new_circuit: {:?}", &new_circuit);

        assert!(circuits.insert(new_circuit.clone()));
        // let new_circuit = circuits.get(&new_circuit).unwrap();
        for junction_box in new_circuit.iter() {
            assert!(map.insert(*junction_box, new_circuit.clone()).is_some());
        }

        assert!(circuits.remove(&circuit_a));
        assert!(circuits.remove(&circuit_b));

        counter += 1;
    }
    // for (i, circuit) in circuits.clone().iter().enumerate() {
    //     println!("{i} (len: {}): {:?}", circuit.len(), circuit);
    // }

    let mut product = 1;
    for _ in 0..3 {
        let max = circuits
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .clone();
        product *= max.len();
        circuits.remove(&max);
    }

    return Some(product);
}

pub fn part_two(input: &str) -> Option<usize> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, todo!());
    }
}
