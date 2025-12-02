advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_at(1);
        let direction = match direction {
            "R" => 1,
            "L" => -1,
            _ => unreachable!(),
        };
        let amount: i32 = amount.parse().unwrap();

        current += (amount % 100) * direction;
        if current > 99 {
            current -= 100;
        }
        if current < 0 {
            current += 100;
        }
        if current == 0 {
            zeros += 1;
        }
    }

    return Some(zeros);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current = 50;
    let mut zeros = 0;
    for line in input.lines() {
        let (direction, amount) = line.split_at(1);
        let direction: i32 = match direction {
            "R" => 1,
            "L" => -1,
            _ => unreachable!(),
        };
        let mut amount = amount.parse().unwrap();

        let next = current + (amount * direction);

        let overflowed = next < 0 || next > 99;
        if overflowed {
            if next < 0 {
                amount -= current;
                if current != 0 {
                    zeros += 1;
                }
            } else if next > 99 {
                amount -= 100 - current;
                zeros += 1;
            }

            zeros += amount / 100;
            amount = amount % 100;

            current = if direction.is_negative() && amount != 0 {
                100 - amount
            } else {
                amount
            };
        } else {
            current = next;
            if next == 0 {
                zeros += 1;
            }
        }
    }

    return Some(zeros as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
