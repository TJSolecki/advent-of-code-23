use::regex::Regex;

fn get_sequences(input: &str) -> Vec<Vec<i32>> {
    let number_regex = Regex::new(r"(-?\d+)").unwrap();
     return input
        .lines()
        .map(|line| {
            let sequence: Vec<i32> = number_regex
                .captures_iter(line)
                .flat_map(|captures| captures.get(1))
                .map(|number_str| number_str.as_str().parse::<i32>())
                .filter_map(Result::ok)
                .collect();
            return sequence;
        })
        .collect();
}

fn get_next_num_in_sequence(sequence: &Vec<i32>) -> i32 {
    if sequence.len() == 0 {
        panic!("sequence is empty");
    }
    if sequence.iter().all(|element| *element == 0) {
        return 0;
    }

    let mut diff_sequence: Vec<i32> = vec![];
    for i in 0..(sequence.len() - 1) {
        diff_sequence.push(sequence[i + 1] - sequence[i]);
    }
    return sequence[sequence.len() - 1] + get_next_num_in_sequence(&diff_sequence);
}

pub fn part1(input: &str) -> i32 {
    let sequences = get_sequences(input);
    return sequences
        .iter()
        .map(|sequence| get_next_num_in_sequence(sequence))
        .sum();
}

fn get_prev_num_in_sequence(sequence: &Vec<i32>) -> i32 {
    if sequence.len() == 0 {
        panic!("sequence is empty");
    }
    if sequence.iter().all(|element| *element == 0) {
        return 0;
    }

    let mut diff_sequence: Vec<i32> = vec![];
    for i in 0..(sequence.len() - 1) {
        diff_sequence.push(sequence[i + 1] - sequence[i]);
    }
    return sequence[0] - get_prev_num_in_sequence(&diff_sequence);
}

pub fn part2(input: &str) -> i32 {
    let sequences = get_sequences(input);
    return sequences
        .iter()
        .map(|sequence| get_prev_num_in_sequence(sequence))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part1(input), 114)
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part2(input), 2);
    }
}
