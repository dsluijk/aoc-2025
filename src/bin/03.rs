advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    for bank_input in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut bank = vec![];
        for bat in bank_input.split("").filter(|&x| !x.is_empty()) {
            bank.push(bat.parse::<i64>().unwrap());
        }

        let mut best: i64 = 0;
        for (i, left) in bank.iter().enumerate() {
            for right in &bank[i + 1..] {
                let new = left * 10 + right;
                if new > best {
                    best = new;
                }
            }
        }

        result += best;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    for bank_input in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut bank = vec![];
        for bat in bank_input.split("").filter(|&x| !x.is_empty()) {
            bank.push(bat.parse::<i64>().unwrap());
        }

        while bank.len() > 12 {
            let mut lowest = 10;
            let mut lowest_idx = 0;
            for (i, bat) in bank.iter().enumerate() {
                if *bat < lowest {
                    lowest = *bat;
                    lowest_idx = i;
                } else if *bat > lowest {
                    break;
                }
            }

            bank.remove(lowest_idx);
        }

        let mut total = 0;
        for bat in bank {
            total *= 10;
            total += bat;
        }

        result += total;
    }

    Some(result)
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
