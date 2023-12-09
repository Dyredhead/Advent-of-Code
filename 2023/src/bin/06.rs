advent_of_code::solution!(6);
//Imports

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<usize> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<usize> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = times
        .iter()
        .zip(distances.into_iter())
        .fold(1, |n, (&time, distance)| {
            n * (0..=time).fold(0, |m, i| {
                if 1 * i * (time - i) > distance {
                    return m + 1;
                } else {
                    return m;
                }
            })
        });

    return Some(n);
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();

    let time: Vec<&str> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect();
    let time: usize = time.join("").parse().unwrap();

    let distance: Vec<&str> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect();
    let distance: usize = distance.join("").parse().unwrap();

    let n: usize = (0..=time).fold(0, |n, i| {
        if 1 * i * (time - i) > distance {
            return n + 1;
        } else {
            return n;
        }
    });

    return Some(n);
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
