use std::fs;   

fn string_to_number(input: &str) -> u64 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("unexpected number!"),
    }
}

fn solve_p2(input: String) -> u64 {

    let numbers: Vec<_> = input
        .lines()
        .map(|line| line.chars()
            .enumerate()
            .filter_map(|(i, _)| match &line[i..] {
                s if s.starts_with("one") => Some("one"),
                s if s.starts_with("two") => Some("two"),
                s if s.starts_with("three") => Some("three"),
                s if s.starts_with("four") => Some("four"),
                s if s.starts_with("five") => Some("five"),
                s if s.starts_with("six") => Some("six"),
                s if s.starts_with("seven") => Some("seven"),
                s if s.starts_with("eight") => Some("eight"),
                s if s.starts_with("nine") => Some("nine"),
                s if s.chars().into_iter().take(1).collect::<Vec<char>>()[0].is_numeric() => Some(&s[0..1]),
                _ => None,
            })
            .collect()
        )
        .collect();
    
    let first_and_last_numbers: Vec<_> = numbers.into_iter().map(|x: Vec<&str>| [x[0], x[x.len() - 1]]).collect();

    let line_numbers: Vec<u64> = first_and_last_numbers
        .into_iter()
        .map(|x| string_to_number(x[0]) * 10 + string_to_number(x[1]))
        .collect();
        
    let total_sum: u64 = line_numbers
        .iter()
        .sum();

    return total_sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_p2(input.join("\n")), 281);
    }

    #[test]
    fn test_part2_test2() {
        let input: Vec<String> = [
            "47eight1", // 41
            "815vd5gnbgone", // 81
            "xbclfszchvone21", // 11
            "7ttwofourthreehjhpjmtwogrng4", // 74
            "twothreeoneseven9qd6", // 26
            "9dhbgmqgr7threekfhzkqqg", // 93
        ]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(solve_p2(input.join("\n")), 326);
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read in the file");

    println!("{:?}", solve_p2(input));
}

