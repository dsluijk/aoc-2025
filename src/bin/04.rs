use advent_of_code::Grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<i64> {
    let line_length = input.chars().position(|c| c == '\n').unwrap();
    let data: Vec<char> = input.replace('\n', "").chars().collect();
    let total_lines = data.len() / line_length;
    let grid: Grid<char> = Grid::new((line_length, total_lines), data);

    let mut result: i64 = 0;
    for i in 0..total_lines {
        for j in 0..line_length {
            if *grid.at_pos((i, j)) != '@' {
                print!(".");
                continue;
            }

            let mut surrounding = 0;

            grid.each_neighbor_pos((i, j), |neigh_pos| {
                if *grid.at_pos(neigh_pos) == '@' {
                    surrounding += 1
                }
            });

            if surrounding < 4 {
                result += 1;
                print!("x");
            } else {
                print!("@")
            }
        }

        print!("\n")
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let line_length = input.chars().position(|c| c == '\n').unwrap();
    let data: Vec<char> = input.replace('\n', "").chars().collect();
    let total_lines = data.len() / line_length;
    let mut grid: Grid<char> = Grid::new((line_length, total_lines), data);

    let mut result: i64 = 0;
    loop {
        let mut removed = 0;

        for i in 0..total_lines {
            for j in 0..line_length {
                if *grid.at_pos((i, j)) != '@' {
                    print!(".");
                    continue;
                }

                let mut surrounding = 0;

                grid.each_neighbor_pos((i, j), |neigh_pos| {
                    if *grid.at_pos(neigh_pos) == '@' {
                        surrounding += 1
                    }
                });

                if surrounding < 4 {
                    removed += 1;
                    let cell: &mut char = grid.at_pos_mut((i, j));
                    *cell = '.';
                    print!("x");
                } else {
                    print!("@")
                }
            }

            print!("\n")
        }

        print!("\n\n");

        if removed == 0 {
            break;
        }
        result += removed;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
