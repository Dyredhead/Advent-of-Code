advent_of_code::solution!(3);
use iter_first_max::IterFirstMaxExt as _;

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let (i1, n1) = line[..line.len() - 1]
            .char_indices()
            .first_max_by_key(|(_, value)| *value)
            .unwrap();

        let n2 = line[i1 + 1..].chars().max().unwrap();
        sum += (n1 as u8 - b'0') as usize * 10 + (n2 as u8 - b'0') as usize;
    }

    return Some(sum as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let mut index = 0;
        // let mut num = String::with_capacity(12);
        for i in (1..=12).rev() {
            let (max_index, max) = line[index..=line.len() - i]
                .char_indices()
                .first_max_by_key(|(_, value)| *value)
                .unwrap();
            index += max_index + 1;
            // num.push(max);
            let max = max as u8 - b'0';
            sum += max as usize * 10_usize.pow(i as u32 - 1);
        }
        // let num = num.parse::<usize>().unwrap();
        // sum += num;
    }
    return Some(sum as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
