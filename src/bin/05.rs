advent_of_code::solution!(5);

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
};
use std::cmp;
use std::ops::RangeInclusive;

fn parse_fresh(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) =
        terminated(separated_pair(u64, tag("-"), u64), newline).parse(input)?;
    Ok((input, (start..=end)))
}

fn parse_ingredients(input: &str) -> IResult<&str, u64> {
    terminated(u64, opt(newline)).parse(input)
}

fn parse_all(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (input, fresh) = many1(parse_fresh).parse(input)?;
    let (input, _) = many1(newline).parse(input)?;
    let (input, ingredients) = many1(parse_ingredients).parse(input)?;

    Ok((input, (fresh, ingredients)))
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    let (_, (fresh, ingredients)) = parse_all(input).unwrap();
    for ingredient in ingredients {
        let mut is_fresh = false;
        for fresh_range in &fresh {
            if ingredient >= *fresh_range.start() && ingredient <= *fresh_range.end() {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    let (_, (mut fresh, _)) = parse_all(input).unwrap();
    fresh.sort_by(|a, b| a.start().cmp(b.start()));

    let mut start: u64 = 0;
    for range in fresh {
        start = cmp::max(start, *range.start());
        if start > *range.end() {
            continue;
        }

        result += range.end() - start + 1;
        start = *range.end() + 1;
    }

    Some(result)
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
        assert_eq!(result, Some(14));
    }
}
