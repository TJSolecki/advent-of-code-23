#[derive(Debug)]
struct NumberInfo {
    row: usize,
    col: usize,
    length: usize,
    number: u32,
}

fn get_number_locations(input: String) -> Vec<NumberInfo> {
    let mut number_locations: Vec<NumberInfo> = vec![];
    input.lines().enumerate().for_each(|(row, line)| {
        let mut start_col: usize = 0;
        let mut number_string: String = "".to_string();
        for (col, char) in line.chars().enumerate() {
            if number_string.len() == 0 && char.is_numeric() {
                number_string = char.to_string();
                start_col = col;
            } else if char.is_numeric() {
                number_string = format!("{}{}", number_string, char);
            } else {
                if number_string.len() > 0 {
                    number_locations.push(NumberInfo {
                        row,
                        col: start_col,
                        length: col - start_col,
                        number: number_string.parse::<u32>().expect("NaN")
                    });
                    number_string = "".to_string();
                }
            }
        }
        if number_string.len() != 0 {
            number_locations.push(NumberInfo {
                row,
                col: start_col,
                length: line.len() - start_col,
                number: number_string.parse::<u32>().expect("NaN")
            });
        }
    });
    return number_locations;
}

fn has_symbol_neighbor(number_info: &NumberInfo, puzzle: &Vec<Vec<char>>) -> bool {
    let start_col = number_info.col;
    let end_col = number_info.col + number_info.length - 1;
    let number_length = number_info.length;
    let row = number_info.row;
    let line_length = puzzle[0].len();
    let num_rows = puzzle.len();

    if start_col != 0 && puzzle[row][start_col - 1] != '.' {
        return true;
    }

    if end_col < line_length - 1 && puzzle[row][end_col + 1] != '.' {
        return true;
    }

    for col in ((start_col as i32 - 1) as i32)..=((start_col + number_length) as i32) {
        if col < 0 || col >= line_length as i32 {
            continue;
        }
        if row != 0 && puzzle[row - 1][col as usize] != '.' {
            return true;
        }
        if row < num_rows - 1 && puzzle[row + 1][col as usize] != '.' {
            return true;
        }
    }

    return false;
}

pub fn part1(input: String) -> u32 {
    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let number_locations: Vec<NumberInfo> = get_number_locations(input);

    return number_locations
        .iter()
        .filter(|number_info| has_symbol_neighbor(number_info, &puzzle))
        .map(|number_info| number_info.number)
        .sum();
}

fn get_gear_ratio(gear_row: usize, gear_col: usize, number_locations: &Vec<NumberInfo>) -> u32 {
    let adjacent_number_locations: Vec<&NumberInfo> = number_locations
        .iter()
        .filter(|number_info| {
            // Check rows
            if !vec![number_info.row, number_info.row + 1].contains(&gear_row)
                && !(number_info.row != 0 && gear_row == number_info.row - 1) {
                return false;
            }

            // Check cols
            if number_info.col != 0 && gear_col == number_info.col - 1 {
                return true;
            }

            if (number_info.col..=number_info.col + number_info.length).contains(&gear_col) {
                return true;
            }

            return false;
        })
        .collect();
    
    if adjacent_number_locations.len() < 2 {
        return 0;
    }

    return adjacent_number_locations
        .iter()
        .map(|number_info| number_info.number)
        .fold(1, |product, number| product * number);
}

pub fn part2(input: String) -> u32 {
    let puzzle: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let number_locations: Vec<NumberInfo> = get_number_locations(input);

    let mut sum: u32 = 0;
    puzzle.iter().enumerate().for_each(|(row, line_arr)| {
        line_arr.iter().enumerate().for_each(|(col, char)| {
            if *char == '*' {
                sum += get_gear_ratio(row, col, &number_locations);
            }
        })
    });
    
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part1(input.to_string()), 4361);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part2(input.to_string()), 467835);
    }
}
