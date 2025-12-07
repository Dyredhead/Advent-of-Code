use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = input.lines().rev();
    let operations = input.next().unwrap().chars().filter(|&c| c != ' ');
    let input: Vec<Vec<&str>> = input
        .rev()
        .map(|line| line.split_ascii_whitespace().collect())
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
    let input: Vec<&AsciiStr> = input.as_ascii_str().unwrap().lines().collect();
    let operations: Vec<AsciiChar> = input
        .last()
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect();

    let input = input[..input.len() - 1].to_vec();

    let input1: Vec<Vec<&AsciiStr>> = input
        .iter()
        .map(|line| {
            line.split(ascii::AsciiChar::Space)
                .filter(|c| !c.is_empty())
                .collect()
        })
        .collect();

    let mut splits: Vec<usize> = Vec::new();
    for col in 0..input1[0].len() - 1 {
        let max = input1
            .iter()
            .map(|line| line[col])
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len();
        splits.push(max);
    }

    let input2: Vec<Vec<&AsciiStr>> = input
        .into_iter()
        .map(|line| {
            let mut line = line.as_slice();
            let mut new: Vec<&AsciiStr> = Vec::new();
            for i in splits.iter() {
                let (left, right) = line.split_at(*i);
                new.push(left.as_ascii_str().unwrap());
                line = &right[1..];
            }
            new.push(line.as_ascii_str().unwrap());
            new
        })
        .collect();

    let mut sum = 0;
    for (index, str) in input2[0].iter().enumerate().rev() {
        let mut accumelator = match operations[index] {
            AsciiChar::Plus => 0,
            AsciiChar::Asterisk => 1,
            _ => unreachable!(),
        };

        for col in (0..str.len()).rev() {
            let mut digits: AsciiString = AsciiString::new();
            for row in 0..input2.len() {
                let char = input2[row][index].chars().nth(col).unwrap();
                if char.is_ascii_digit() {
                    digits.push(char);
                }
            }
            let num: usize = digits.to_string().parse().unwrap();
            match operations[index] {
                AsciiChar::Plus => accumelator += num,
                AsciiChar::Asterisk => accumelator *= num,
                _ => unreachable!(),
            };
        }
        sum += accumelator;
    }

    return Some(sum);
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
        assert_eq!(result, Some(3263827));
    }
}
