use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

advent_of_code::solution!(8);

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
    return f32::sqrt((delta_x * delta_x + delta_y * delta_y + delta_z * delta_z) as f32);
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

    println!("circuits: {:?}\n", &circuits);

    let mut map: HashMap<JunctionBox, Circuit> = junction_boxes
        .iter()
        .map(|junction_box| (*junction_box, Circuit::from([*junction_box])))
        .collect();

    println!("map: {:?}\n", &map);

    let mut dp: HashSet<(JunctionBox, JunctionBox)> = HashSet::new();
    for _ in 0..7 {
        // maybe causes problem since what if these are the min
        // let mut min_junction_boxes = (junction_boxes[0], junction_boxes[1]);
        // let mut min_distance = distance(min_junction_boxes.0, min_junction_boxes.1);
        let mut min_junction_boxes = None;
        let mut min_distance = None;
        for (i, &a) in junction_boxes.iter().enumerate() {
            for &b in junction_boxes.iter().skip(i + 1) {
                if a == b || map.get(&a).unwrap().contains(&b) {
                    continue;
                }

                let distance = distance(a, b);
                if min_distance.is_none() || distance < min_distance.unwrap() {
                    min_distance = Some(distance);
                    min_junction_boxes = Some((a, b));
                }
            }
        }
        // println!("map before: {:#?}", &map);

        let min_junction_boxes = min_junction_boxes.unwrap();
        dp.insert(min_junction_boxes);
        let a = min_junction_boxes.0;
        let b = min_junction_boxes.1;

        // println!("a: {a:?}");
        // println!("b: {b:?}");

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

        // println!("map after: {:#?}", &map);
        // println!();
    }
    for (i, circuit) in circuits.clone().iter().enumerate() {
        println!("{i} (len: {}): {:?}", circuit.len(), circuit);
    }

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
