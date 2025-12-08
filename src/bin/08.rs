use memoize::memoize;
use std::collections::HashSet;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

advent_of_code::solution!(8);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Point(i64, i64, i64);

#[memoize]
fn pair_distance(a: Point, b: Point) -> i64 {
    let mut total = 0;
    total += (a.0 - b.0).abs().pow(2);
    total += (a.1 - b.1).abs().pow(2);
    total += (a.2 - b.2).abs().pow(2);

    f64::sqrt(total as f64) as i64
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut coords = vec![];
    let mut totals = vec![];

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        let z = split.next().unwrap().parse::<i64>().unwrap();
        coords.push(Point(x, y, z));
        totals.push(0);
    }

    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());
    let mut seen = HashSet::new();
    let connections = match coords.len() {
        20 => 10,
        1000 => 1000,
        _ => unreachable!(),
    };

    for _ in 0..connections {
        let mut shortest = (0, 0);
        let mut shortest_dist = 99999999999;

        for (i, coord_a) in coords.iter().enumerate() {
            for (j_off, coord_b) in coords[i + 1..].iter().enumerate() {
                let j = j_off + i + 1;
                if seen.contains(&(i, j)) {
                    continue;
                }

                let dist = pair_distance(*coord_a, *coord_b);
                if dist < shortest_dist {
                    shortest = (i, j);
                    shortest_dist = dist;
                }
            }
        }

        uf.union(shortest.0, shortest.1);
        seen.insert(shortest);
    }

    for i in 0..coords.len() {
        totals[uf.find(i)] += 1;
    }

    totals.sort_by(|a, b| b.cmp(a));
    Some(totals[0] * totals[1] * totals[2])
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut coords = vec![];

    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();
        let z = split.next().unwrap().parse::<i64>().unwrap();
        coords.push(Point(x, y, z));
    }

    let mut uf = QuickUnionUf::<UnionBySize>::new(coords.len());
    let mut last = (0, 0);

    loop {
        let mut shortest = (0, 0);
        let mut shortest_dist = 99999999999;
        let mut is_connected = true;

        for (i, coord_a) in coords.iter().enumerate() {
            for (j_off, coord_b) in coords[i + 1..].iter().enumerate() {
                let j = j_off + i + 1;
                if uf.find(i) == uf.find(j) {
                    continue;
                }

                let dist = pair_distance(*coord_a, *coord_b);
                if dist < shortest_dist {
                    shortest = (i, j);
                    shortest_dist = dist;
                }
            }

            if uf.find(0) != uf.find(i) {
                is_connected = false;
            }
        }

        if is_connected {
            break;
        }

        uf.union(shortest.0, shortest.1);
        last = shortest;
    }

    Some(coords[last.0].0 * coords[last.1].0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
