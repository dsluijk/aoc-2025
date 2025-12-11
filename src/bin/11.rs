use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;
    let mut ports = HashMap::new();

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut split = line.splitn(2, ": ");
        let key = split.next().unwrap();
        let mut dests = vec![];

        for dest in split.next().unwrap().split(" ").filter(|&x| !x.is_empty()) {
            dests.push(dest);
        }

        ports.insert(key, dests);
    }

    let mut seen = HashSet::new();
    let mut pq = PriorityQueue::new();
    pq.push(vec!["you"], Reverse(0));

    while let Some((path, priority)) = pq.pop() {
        if seen.contains(&path) {
            continue;
        }

        for dest in ports.get(path.last().unwrap()).unwrap() {
            if path.contains(dest) {
                continue;
            }

            if *dest == "out" {
                result += 1;
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(dest);
            pq.push(new_path, Reverse(priority.0 + 1));
        }

        seen.insert(path);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ports = HashMap::new();

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut split = line.splitn(2, ": ");
        let key = split.next().unwrap();
        let mut dests = vec![];

        for dest in split.next().unwrap().split(" ").filter(|&x| !x.is_empty()) {
            dests.push(dest);
        }

        ports.insert(key, dests);
    }

    let mut checked: HashMap<(&str, bool, bool), u64> = HashMap::new();
    fn check_port<'a>(
        ports: &HashMap<&str, Vec<&'a str>>,
        checked: &mut HashMap<(&'a str, bool, bool), u64>,
        port: &'a str,
        mut dac: bool,
        mut fft: bool,
    ) -> u64 {
        if port == "out" {
            return if dac && fft { 1 } else { 0 };
        }

        if port == "dac" {
            dac = true;
        }

        if port == "fft" {
            fft = true;
        }

        let mut total = 0;
        for dest in ports.get(port).unwrap() {
            total += match checked.get(&(dest, dac, fft)) {
                Some(&v) => v,
                _ => check_port(&ports, checked, dest, dac, fft),
            };
        }

        checked.insert((port, dac, fft), total);
        total
    }

    Some(check_port(&ports, &mut checked, "svr", false, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
