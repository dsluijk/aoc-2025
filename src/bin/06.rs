advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<i64> {
    let mut operators = vec![];
    let lines: Vec<&str> = input.split("\n").filter(|&x| !x.is_empty()).collect();
    let mut columns: Vec<i64> = vec![];
    for operator in lines.last()?.split(" ").filter(|&x| !x.is_empty()) {
        operators.push(operator);
        columns.push(match operator {
            "*" => 1,
            "+" => 0,
            _ => 99999999,
        });
    }

    for line in &lines[..lines.len() - 1] {
        for (i, cell) in line.split(" ").filter(|&x| !x.is_empty()).enumerate() {
            let value = cell.parse::<i64>().unwrap();
            match operators[i] {
                "*" => columns[i] *= value,
                "+" => columns[i] += value,
                _ => unreachable!(),
            };
        }
    }

    let mut result: i64 = 0;
    for column in columns {
        result += column;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines: Vec<&str> = input.split("\n").filter(|&x| !x.is_empty()).collect();
    let mut result = 0;
    let mut curr_total = 0;
    let mut curr_op = ' ';

    for i in 0..lines.last()?.len() {
        let op = lines.last()?.chars().nth(i)?;

        if op != ' ' {
            result += curr_total;
            curr_total = match op {
                '*' => 1,
                '+' => 0,
                _ => unreachable!(),
            };
            curr_op = op;
        }

        let mut column: Vec<char> = vec![];
        for line in &lines[0..lines.len() - 1] {
            let c = line.chars().nth(i)?;
            if c == ' ' {
                continue;
            }

            column.push(c);
        }

        if column.len() == 0 {
            continue;
        }

        let col = column
            .into_iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        match curr_op {
            '*' => curr_total *= col,
            '+' => curr_total += col,
            _ => unreachable!(),
        };
    }

    result += curr_total;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
