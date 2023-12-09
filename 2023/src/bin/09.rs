advent_of_code::solution!(9);

static NEXT: bool = true;
static PREV: bool = false;

pub fn part_zero(input: &str, value: bool) -> i32 {
    let lines: Vec<&str> = input.lines().collect::<Vec<_>>();

    let mut histories: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .map(|x: Vec<&str>| x.iter().map(|y| y.parse().unwrap()).collect())
        .collect();

    let sum: i32 = histories
        .iter_mut()
        .fold(0, |sum: i32, history: &mut Vec<i32>| {
            if value == NEXT {
                sum + next_value(history)
            } else {
                sum + prev_value(history)
            }
        });

    return sum;
}
fn next_value(history_old: &mut Vec<i32>) -> i32 {
    if history_old.iter().all(|&x| x == 0) {
        history_old.push(0);

        return *history_old.last().unwrap();
    } else {
        let mut history_new: Vec<i32> = history_old.windows(2).map(|x| x[1] - x[0]).collect();
        next_value(&mut history_new);
        history_old.push(*history_old.last().unwrap() + *history_new.last().unwrap());

        return *history_old.last().unwrap();
    }
}

fn prev_value(history_old: &mut Vec<i32>) -> i32 {
    if history_old.iter().all(|&x| x == 0) {
        history_old.insert(0, 0);

        return *history_old.first().unwrap();
    } else {
        let mut history_new: Vec<i32> = history_old.windows(2).map(|x| x[1] - x[0]).collect();
        prev_value(&mut history_new);
        history_old.insert(
            0,
            *history_old.first().unwrap() - *history_new.first().unwrap(),
        );

        return *history_old.first().unwrap();
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    return Some(part_zero(input, NEXT));
}
pub fn part_two(input: &str) -> Option<i32> {
    return Some(part_zero(input, PREV));
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
