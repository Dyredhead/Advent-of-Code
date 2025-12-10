#![feature(hash_set_entry)]

advent_of_code::solution!(8);

use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashMap, HashSet},
    rc::Rc,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

type Circuit<'a> = BTreeSet<&'a JunctionBox>;

fn distance(a: &JunctionBox, b: &JunctionBox) -> f32 {
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

    let mut circuits: HashSet<Rc<Circuit>> = junction_boxes
        .iter()
        .map(|junction_box| Rc::new(Circuit::from_iter([junction_box])))
        .collect();

    let mut map: HashMap<&JunctionBox, Rc<Circuit>> = circuits
        .iter()
        .map(|circuit| (*circuit.first().unwrap(), Rc::clone(circuit)))
        .collect();

    let mut pairs: Vec<(&JunctionBox, &JunctionBox, f32)> =
        Vec::with_capacity(junction_boxes.len().pow(2) / 2);

    for (i, a) in junction_boxes.iter().enumerate() {
        for b in junction_boxes.iter().skip(i + 1) {
            pairs.push((a, b, distance(a, b)));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut pairs = pairs.into_iter();

    let mut counter = 0;
    while counter < 1000 - 1 {
        counter += 1;
        let pair = pairs.next().unwrap();

        if map.get(pair.0).unwrap().contains(pair.1) {
            continue;
        }

        let a = pair.0;
        let b = pair.1;

        let circuit_a = map.get(a).unwrap();
        let circuit_b = map.get(b).unwrap();

        assert!(circuits.remove(circuit_a));
        assert!(circuits.remove(circuit_b));

        let new_circuit = circuit_a.union(circuit_b).cloned().collect();

        // let new_circuit = [circuit_a, circuit_b]
        //     .into_iter()
        //     .flat_map(|rc| rc.iter().copied())
        //     .collect();

        let new_circuit = circuits.get_or_insert(Rc::new(new_circuit));
        for junction_box in new_circuit.iter() {
            assert!(map.insert(junction_box, Rc::clone(new_circuit)).is_some());
        }
    }

    let mut circuits = Vec::from_iter(circuits);
    circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

    let product = circuits
        .iter()
        .take(3)
        .fold(1, |acc, circuit| acc * circuit.len());

    return Some(product);
}

pub fn part_two(input: &str) -> Option<usize> {
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

    let mut circuits: HashSet<Rc<Circuit>> = junction_boxes
        .iter()
        .map(|junction_box| Rc::new(Circuit::from_iter([junction_box])))
        .collect();

    let mut map: HashMap<&JunctionBox, Rc<Circuit>> = circuits
        .iter()
        .map(|circuit| (*circuit.first().unwrap(), Rc::clone(circuit)))
        .collect();

    let mut pairs: Vec<(&JunctionBox, &JunctionBox, f32)> =
        Vec::with_capacity(junction_boxes.len().pow(2) / 2);

    for (i, a) in junction_boxes.iter().enumerate() {
        for b in junction_boxes.iter().skip(i + 1) {
            pairs.push((a, b, distance(a, b)));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let pairs = pairs.into_iter();

    let mut last_pair = None;
    for pair in pairs {
        if map.get(pair.0).unwrap().contains(pair.1) {
            continue;
        }
        last_pair = Some(pair);

        let a = pair.0;
        let b = pair.1;

        let circuit_a = map.get(a).unwrap();
        let circuit_b = map.get(b).unwrap();

        assert!(circuits.remove(circuit_a));
        assert!(circuits.remove(circuit_b));

        let new_circuit = circuit_a.union(circuit_b).cloned().collect();

        let new_circuit = circuits.get_or_insert(Rc::new(new_circuit));
        for junction_box in new_circuit.iter() {
            assert!(map.insert(junction_box, Rc::clone(new_circuit)).is_some());
        }
    }

    let (a, b, _) = last_pair.unwrap();

    return Some(a.x * b.x);
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
        assert_eq!(result, Some(25272));
    }
}
