use std::ops::RangeInclusive;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split(',')
        .map(|range: &str| range.split_once('-').unwrap())
        .map(|range| {
            let start: usize = range.0.parse().unwrap();
            let end: usize = range.1.parse().unwrap();
            RangeInclusive::new(start, end)
        });

    let mut invalid_ids_sum = 0;

    for range in ranges {
        let start: usize = {
            let start = *range.start();
            let mut exponent = (start as f32).log10().trunc() as u32;
            if exponent % 2 == 1 {
                start
            } else {
                // should give closest number > start that is of the form 1(0...)1(0...)
                exponent += 1;
                10_usize.pow(exponent) + 10_usize.pow(exponent / 2)
            }
        };

        let end: usize = {
            let end = *range.end();
            let exponent = (end as f32).log10().trunc() as u32;
            if exponent % 2 == 1 {
                end
            } else {
                // should give closest number < end that is of the form 9...
                10_usize.pow(exponent) - 1
            }
        };

        let (start, end) = (start.to_string(), end.to_string());
        let (start_length, end_length) = (
            (start.len() as f32 / 2_f32).round() as usize,
            (end.len() as f32 / 2_f32).round() as usize,
        );

        let start: usize = start[..start_length].parse().unwrap();
        let end: usize = end[..end_length].parse().unwrap();

        for i in start..=end {
            let i: usize = (i.to_string() + &i.to_string()).parse().unwrap();
            if range.contains(&i) {
                invalid_ids_sum += i;
            }
        }
    }

    return Some(invalid_ids_sum as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input
        .split(',')
        .map(|range: &str| range.split_once('-').unwrap())
        .map(|range| {
            let start: usize = range.0.parse().unwrap();
            let end: usize = range.1.parse().unwrap();
            RangeInclusive::new(start, end)
        });

    let mut invalid_ids_sum = 0;

    for range in ranges {
        for i in range {
            let i_str = i.to_string();
            for j in 1..((i_str.len() / 2) + 1) {
                if i_str.len() % j == 0 {
                    if i_str == i_str[..j].repeat(i_str.len() / j) {
                        invalid_ids_sum += i;
                        break;
                    }
                }
            }
        }
    }

    return Some(invalid_ids_sum as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
