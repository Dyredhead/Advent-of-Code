use std::{collections::HashSet, ops::RangeInclusive};

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
        let digit_count_range =
            RangeInclusive::new(count_digits(*range.start()), count_digits(*range.end()));
        let mut hash_set = HashSet::<usize>::new();
        for digit_count in digit_count_range.clone().into_iter() {
            let mut seed = 1;
            loop {
                if count_digits(seed) > digit_count / 2 {
                    break;
                }

                // if the seed_digits cant perfectly fit into digit_count, then a repeating number is not possible
                let seed_digit_count = count_digits(seed);
                if digit_count % seed_digit_count != 0 {
                    seed += 1;
                    continue;
                }

                let repetitions = digit_count / seed_digit_count;
                let id = repeat(seed, repetitions);

                if hash_set.contains(&id) {
                    seed += 1;
                } else {
                    if range.contains(&id) {
                        invalid_ids_sum += id;
                        hash_set.insert(id);
                        seed += 1;
                    } else if id > *range.end() {
                        seed = 10_usize.pow(count_digits(seed) as u32);
                        // maybe could be improved but the below wont work since it can skip some numbers
                        // seed = first_n_digits_of_number(*range.start(), count_digits(seed) + 1);
                    } else {
                        seed += 1;
                    }
                }
            }
        }
    }

    return Some(invalid_ids_sum as u64);
}

fn count_digits(number: usize) -> usize {
    if number == 1 {
        return 1;
    }
    let log = (number as f32).log10();
    let digits = log.ceil() as usize;
    if log.fract() == 0_f32 {
        return digits + 1;
    }
    return digits;
}

fn repeat(number: usize, repetitions: usize) -> usize {
    let digit_count = count_digits(number);
    let mut sum: usize = 0;
    for i in 0..repetitions {
        sum += number * 10_usize.pow((i * digit_count) as u32);
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
