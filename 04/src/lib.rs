use regex::Regex;

#[derive(Debug)]
struct ScratchCard {
    card_num: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn get_scratch_cards(input: &str) -> Vec<ScratchCard> { let card_regex = Regex::new(r"Card\s+(?<card_num>\d+):").unwrap();
    let number_regex = Regex::new(r"\b(\d+)\b").unwrap();
    return input.lines().map(|line| {
        let Some(card_num_match) = card_regex.captures(line) else {
            panic!("Could not match card_num");
        };
        let card_num: u32 = card_num_match["card_num"].parse::<u32>().unwrap_or(0);
        let line_arr: Vec<&str> = line.split(":").collect();
        let all_numbers: &str = line_arr[1];
        let number_partitions: Vec<&str> = all_numbers.split("|").collect();
        let numbers: Vec<u32> = number_regex
            .captures_iter(number_partitions[0])
            .flat_map(|captures| captures.get(1))
            .map(|number_str| number_str.as_str().parse::<u32>())
            .filter_map(Result::ok)
            .collect();
        
        let winning_numbers: Vec<u32> = number_regex
            .captures_iter(number_partitions[1])
            .flat_map(|captures| captures.get(1))
            .map(|number_str| number_str.as_str().parse::<u32>())
            .filter_map(Result::ok)
            .collect();

        return ScratchCard { card_num, winning_numbers, numbers };
    })
    .collect();

}

fn get_num_matches(scratch_card: &ScratchCard) -> u32 {
    return scratch_card.numbers
        .iter()
        .filter(|number| scratch_card.winning_numbers.contains(*number))
        .count() as u32;
}

fn get_scratch_card_points(scratch_card: &ScratchCard) -> u32 {
    let num_matches: u32 = get_num_matches(scratch_card);
    if num_matches == 0 {
        return 0;
    }
    return (2 as u32).pow(num_matches - 1);
}

fn get_total_points(scratch_cards: Vec<ScratchCard>) -> u32 {
    return scratch_cards
        .iter()
        .map(|scratch_card| get_scratch_card_points(scratch_card))
        .sum();
}

pub fn part1(input: &str) -> u32 {
    return get_total_points(get_scratch_cards(input));
}

fn get_num_scratch_card_copies(scratch_card: &ScratchCard, scratch_cards: &Vec<ScratchCard>) -> u32 {
    let index: usize = scratch_card.card_num.clone() as usize - 1;
    let num_matches: u32 = get_num_matches(scratch_card);
    if scratch_card.card_num == scratch_cards.len() as u32 {
        return 1;
    }
    if num_matches == 0 {
        return 1;
    }
    return 1 + ((index+1)..=index+num_matches as usize)
        .into_iter()
        .map(|i| get_num_scratch_card_copies(&scratch_cards[i], scratch_cards))
        .sum::<u32>();
}

pub fn part2(input: &str) -> u32 {
    let scratch_cards: Vec<ScratchCard> = get_scratch_cards(input);
    return scratch_cards
        .iter()
        .map(|scratch_card| get_num_scratch_card_copies(scratch_card, &scratch_cards))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part2(input), 30);
    }
}
