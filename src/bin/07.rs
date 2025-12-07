use advent_of_code::Grid;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    let line_length = input.chars().position(|c| c == '\n').unwrap() + 1;
    let data: Vec<char> = input.replace('\n', "").chars().collect();
    let total_lines = data.len() / line_length;
    let mut grid: Grid<char> = Grid::new((line_length, total_lines), data);

    let mut result: i64 = 0;

    for i in 0..line_length {
        if *grid.at_pos((0, i)) == 'S' {
            *grid.at_pos_mut((1, i)) = '|';
            break;
        }
    }

    for i in 1..total_lines {
        for j in 0..(line_length - 1) {
            if *grid.at_pos((i, j)) == '.' && *grid.at_pos((i - 1, j)) == '|' {
                *grid.at_pos_mut((i, j)) = '|';
                continue;
            }
        }

        for j in 0..(line_length - 1) {
            if *grid.at_pos((i, j)) != '^' || *grid.at_pos((i - 1, j)) != '|' {
                continue;
            }

            result += 1;

            if *grid.at_pos((i, j - 1)) == '.' {
                *grid.at_pos_mut((i, j - 1)) = '|';
            }

            if *grid.at_pos((i, j + 1)) == '.' {
                *grid.at_pos_mut((i, j + 1)) = '|';
            }
        }

        for j in 0..(line_length - 1) {
            print!("{}", grid.at_pos((i, j)));
        }

        println!(" {}", result);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let line_length = input.chars().position(|c| c == '\n').unwrap() + 1;
    let data: Vec<char> = input.replace('\n', "").chars().collect();
    let total_lines = data.len() / line_length;
    let mut grid: Grid<char> = Grid::new((line_length, total_lines), data);
    let mut totals = Vec::new();

    for _ in 0..total_lines {
        let row_vec = std::iter::repeat_n(0, line_length).collect::<Vec<_>>();
        totals.push(row_vec);
    }

    for i in 0..line_length {
        if *grid.at_pos((0, i)) == 'S' {
            *grid.at_pos_mut((1, i)) = '|';
            totals[1][i] = 1;
            break;
        }
    }

    for i in 1..total_lines {
        for j in 0..(line_length - 1) {
            if *grid.at_pos((i, j)) == '.' && *grid.at_pos((i - 1, j)) == '|' {
                *grid.at_pos_mut((i, j)) = *grid.at_pos((i - 1, j));
                totals[i][j] = totals[i - 1][j];
                continue;
            }
        }

        for j in 0..(line_length - 1) {
            if *grid.at_pos((i, j)) != '^' || *grid.at_pos((i - 1, j)) != '|' {
                continue;
            }

            *grid.at_pos_mut((i, j - 1)) = '|';
            totals[i][j - 1] += totals[i - 1][j];

            *grid.at_pos_mut((i, j + 1)) = '|';
            totals[i][j + 1] += totals[i - 1][j];
        }

        for j in 0..(line_length - 1) {
            print!("{}", grid.at_pos((i, j)));
        }

        println!();
    }

    Some(totals.last()?.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
