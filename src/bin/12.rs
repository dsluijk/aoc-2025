advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;

    let sections: Vec<&str> = input.split("\n\n").filter(|&x| !x.is_empty()).collect();
    let mut shapes = vec![];
    for shape in sections[..sections.len() - 1].iter() {
        let grid = &shape
            .split("\n")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>()[1..];

        let mut shape_total = 0;
        for line in grid {
            shape_total += line
                .match_indices("#")
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>()
                .len();
        }
        shapes.push(shape_total as i64);
    }

    for line in sections[sections.len() - 1]
        .split("\n")
        .filter(|&x| !x.is_empty())
    {
        let mut split = line.splitn(2, ": ");
        let size = split.next().unwrap();
        let mut sizes = size.splitn(2, "x");
        let x = sizes.next().unwrap().parse::<i64>().unwrap();
        let y = sizes.next().unwrap().parse::<i64>().unwrap();

        let mut rem_space = x * y;
        for (shape_i, count) in split
            .next()
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .enumerate()
        {
            let shape_count = count.parse::<i64>().unwrap();
            rem_space -= shape_count * shapes[shape_i];
        }

        if rem_space >= 0 {
            result += 1;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
