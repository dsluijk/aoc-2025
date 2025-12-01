advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut dial: i64 = 50;
    let mut at_zero: i64 = 0;

    for instr in input.split("\n").filter(|&x| !x.is_empty()) {
        let offset = &instr[1..].parse::<i64>().unwrap();
        dial = if instr.starts_with("L") {
            dial - offset
        } else {
            dial + offset
        }
        .rem_euclid(100);

        if dial == 0 {
            at_zero += 1;
        }
    }

    Some(at_zero)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut dial: i64 = 50;
    let mut at_zero: i64 = 0;

    for instr in input.split("\n").filter(|&x| !x.is_empty()) {
        let offset = &instr[1..].parse::<i64>().unwrap();

        for _ in 0..*offset {
            if instr.starts_with("L") {
                dial -= 1;
            } else {
                dial += 1;
            }

            dial = dial.rem_euclid(100);

            if dial == 0 {
                at_zero += 1;
            }
        }
    }

    Some(at_zero)
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

    #[test]
    fn test_thousand() {
        let result = part_two("R1000");
        assert_eq!(result, Some(10));

        let result = part_two("L1000");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_zero_no_overflow() {
        let result = part_two("L50");
        assert_eq!(result, Some(1));
    }
}
