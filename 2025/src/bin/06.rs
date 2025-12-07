advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.lines().rev();
    let operations = input.next().unwrap().chars().filter(|&c| c != ' ');
    let input: Vec<Vec<&str>> = input
        .rev()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut sum = 0;
    for (index, operation) in operations.enumerate() {
        let acc = match operation {
            '+' => input.iter().fold(0, |acc: usize, num| {
                acc + num[index].parse::<usize>().unwrap()
            }),
            '*' => input.iter().fold(1, |acc: usize, num| {
                acc * num[index].parse::<usize>().unwrap()
            }),
            _ => unreachable!(),
        };
        sum += acc;
    }
    return Some(sum);
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, todo!());
    }
}
