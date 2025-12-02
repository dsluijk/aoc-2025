advent_of_code::solution!(2);

use fancy_regex::Regex;

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    for range in input
        .replace("\n", "")
        .split(",")
        .filter(|&x| !x.is_empty())
    {
        let mut split = range.split("-");
        let start = split.next().unwrap().parse::<i64>().unwrap();
        let end = split.next().unwrap().parse::<i64>().unwrap();

        for i in start..=end {
            let str = i.to_string();
            if str.len() % 2 == 1 {
                continue;
            }

            let (a, b) = str.split_at(str.len() / 2);
            if a == b {
                result += i;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64 = 0;
    let pattern = Regex::new(r"^([0-9].*)\1{1,}$").unwrap();

    for range in input
        .replace("\n", "")
        .split(",")
        .filter(|&x| !x.is_empty())
    {
        let mut split = range.split("-");
        let start = split.next().unwrap().parse::<i64>().unwrap();
        let end = split.next().unwrap().parse::<i64>().unwrap();

        for i in start..=end {
            let str = i.to_string();
            if pattern.is_match(&str).unwrap() {
                result += i;
            }
        }
    }

    Some(result)
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
