use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Reveal {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    reveals: Vec<Reveal>,
}

fn get_game_data(input: &String) -> Vec<Game> {
    let game_id_regex = Regex::new(r"Game (?<game_id>\d+): ").unwrap();
    let num_red_regex = Regex::new(r"(?<num_red>\d+) red").unwrap();
    let num_blue_regex = Regex::new(r"(?<num_blue>\d+) blue").unwrap();
    let num_green_regex = Regex::new(r"(?<num_green>\d+) green").unwrap();

    let games: Vec<Game> = input.lines().map(|line| {
        let game_id_match = game_id_regex.captures(line).expect("No game_id match from line");
        let game_id = game_id_match["game_id"].parse::<u32>().expect("Game id is not a number");

        let reveal_strings = line.split(";");
        let reveals: Vec<Reveal> = reveal_strings.map(|reveal_string| {
            let num_red_cap = num_red_regex.captures(reveal_string);
            let num_blue_cap = num_blue_regex.captures(reveal_string);
            let num_green_cap = num_green_regex.captures(reveal_string);

            let mut num_red: u32 = 0;
            let mut num_blue: u32 = 0;
            let mut num_green: u32 = 0;

            if let Some(num_red_match) = num_red_cap {
                num_red = num_red_match["num_red"].parse::<u32>().unwrap_or(0);
            }

            if let Some(num_blue_match) = num_blue_cap {
                num_blue = num_blue_match["num_blue"].parse::<u32>().unwrap_or(0);
            }

            if let Some(num_green_match) = num_green_cap {
                num_green = num_green_match["num_green"].parse::<u32>().unwrap_or(0);
            }

            return Reveal { num_red, num_green, num_blue };
        }).collect();

        return Game { game_id, reveals };
    }).collect();

    return games;
}

fn part_1(input: &String) -> u32 {
    let max_red_cubes: u32 = 12;
    let max_green_cubes: u32 = 13;
    let max_blue_cubes: u32 = 14;

    let games: Vec<Game> = get_game_data(input);

    let sum: u32 = games
        .iter()
        .filter(|game| {
            let has_invalid_reveal: _ = game.reveals
                .iter()
                .any(|reveal| {
                    reveal.num_red > max_red_cubes || reveal.num_blue > max_blue_cubes || reveal.num_green > max_green_cubes
                });
            return !has_invalid_reveal;
        })
        .map(|game| game.game_id)
        .sum();

    return sum;
}

fn get_game_power(reveals: &Vec<Reveal>) -> u32 {
    let max_red: u32 = reveals.iter().fold(0, |max, reveal| {
        if reveal.num_red > max { return reveal.num_red } else { return max }
    });
    let max_blue: u32 = reveals.iter().fold(0, |max, reveal| {
        if reveal.num_blue > max { return reveal.num_blue } else { return max }
    });
    let max_green: u32 = reveals.iter().fold(0, |max, reveal| {
        if reveal.num_green > max { return reveal.num_green } else { return max }
    });

    return max_red * max_blue * max_green;
}

fn part_2(input: &String) -> u32 {
    let games: Vec<Game> = get_game_data(input);
    let total_power: u32 = games
        .iter()
        .map(|game| get_game_power(&game.reveals))
        .sum();
    return total_power;
}

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read in the file");
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        todo!();
    }
}
