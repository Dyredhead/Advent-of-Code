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
        for id in range {
            let digits = digits(id);
            for j in 1..((digits / 2) + 1) {
                if digits % j == 0 {
                    if id == repeat(first_n_digits_of_number(id, j), digits / j) {
                        invalid_ids_sum += id;
                        break;
                    }
                }
            }
        }
    }

    return Some(invalid_ids_sum as u64);
}

fn digits(n: usize) -> usize {
    if n == 1 {
        return 1;
    }
    let log = (n as f32).log10();
    let digits = log.ceil() as usize;
    if log.fract() == 0_f32 {
        return digits + 1;
    }
    return digits;
}

fn first_n_digits_of_number(number: usize, n: usize) -> usize {
    let digits = digits(number);
    let first_n_digits = number / 10_usize.pow((digits - n) as u32);
    return first_n_digits;
}

fn repeat(n: usize, times: usize) -> usize {
    let digits = digits(n);
    let mut sum: usize = 0;
    for i in 0..times {
        sum += n * 10_usize.pow((i * digits) as u32);
    }
    return sum;
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
