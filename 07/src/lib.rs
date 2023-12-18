use::std::collections::HashMap;
#[derive(Debug, Eq, PartialEq)]
struct CamelHand {
    bid: u32,
    hand: Vec<char>,
}

fn get_card_strength(card: &char) -> u32 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("card {:?} provied got get_card_strength not found", card)
    }
}
fn get_hand_strength(hand: &Vec<char>) -> u32 {
    let mut card_counts = HashMap::<char, u32>::new();
    for card in hand {
        if let Some(current_count) = card_counts.get(card) {
            card_counts.insert(*card, *current_count + 1);
        } else {
            card_counts.insert(*card, 1);
        }
    }

    if let Some(num_jokers) = card_counts.get(&'J') {
        let mut card_counts_entries: Vec<(&char, &u32)> = card_counts.iter().collect();
        card_counts_entries.sort_by(|a,b| (b.1).cmp(a.1));
        let (most_freq_card, num_occurrences) = card_counts_entries[0];
        if most_freq_card == &'J' {
            if card_counts_entries.len() != 1 {
                let (second_most_freq_card, num_seccond_most_occurrances) = card_counts_entries[1];
                card_counts.insert(*second_most_freq_card, *num_seccond_most_occurrances + num_jokers);
                card_counts.insert('J', 0);
            }
        } else {
            card_counts.insert(*most_freq_card, *num_occurrences + num_jokers);
            card_counts.insert('J', 0);
        }
    }

    let mut matches: Vec<u32> = card_counts.iter().map(|(_, count)| *count).collect();
    matches.sort();
    matches.reverse();
    match matches[0] {
        5  => return 7, // 5 of a kind
        4 => return 6, // 4 of a kind
        3 => {
            if matches[1] == 2 { // full house
                return 5;
            }
            return 4; // 3 of a kind
        },
        2 => {
            if matches[1] == 2 { // two-pair
                return 3
            }
            return 2; // one-pair
        },
        1 => return 1, // high-card
        _ => panic!("can't get hand strenth for matches: {:?}", matches)
    }
}

fn cmp_cards(card: &char, other_card: &char) -> std::cmp::Ordering {
    return get_card_strength(card).cmp(&get_card_strength(other_card))
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let our_hand_strength = get_hand_strength(&self.hand);
        let their_hand_strength = get_hand_strength(&other.hand);
        match our_hand_strength.cmp(&their_hand_strength) {
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                let our_hand = &self.hand;
                let their_hand = &other.hand;
                for i in 0..our_hand.len() {
                    let ord = cmp_cards(&our_hand[i], &their_hand[i]);
                    if ord == std::cmp::Ordering::Equal {
                        continue;
                    }
                    return ord;
                }
                panic!("All cards match, the hands are equal");
            }
        }
        
    }
}

impl PartialOrd for CamelHand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_camel_hands(input: &str) -> Vec<CamelHand> {
    return input
        .lines()
        .map(|line| {
            let hand_and_bid: Vec<String> = line.split(" ").take(2).map(|partition| partition.to_string()).collect();
            let hand: Vec<char> = hand_and_bid[0].chars().collect();
            let bid: u32 = hand_and_bid[1].parse().unwrap_or_else(|err| panic!("Err: {:?}", err));
            return CamelHand { hand, bid };
        })
        .collect();
}

pub fn part1(input: &str) -> u32 {
    let mut camel_hands = get_camel_hands(input);
    camel_hands.sort();
    return camel_hands
        .iter()
        .enumerate()
        .map(|(i, camel_hand)| (i as u32 +1) * camel_hand.bid)
        .sum();
}

pub fn part2(input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_example() {
    //     let input = include_str!("../test_input.txt");

    //     assert_eq!(part1(input), 6440)
    // }

    // #[test]
    // fn part2_example() {
    //     let input = include_str!("../test_input.txt");

    //     assert_eq!(part2(input), 5905);
    // }

    #[test]
    fn test_hand_strenth() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            'J',
            'J',
            'J',
            'J',
        ]), 7);
    }

    #[test]
    fn test_hand_strenth_2() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            'J',
            'J',
            'J',
            'K',
        ]), 7);
    }

    #[test]
    fn test_hand_strenth_3() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            'J',
            'J',
            'Q',
            'K',
        ]), 6);
    }

    #[test]
    fn test_hand_strenth_4() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            '4',
            '6',
            'Q',
            'K',
        ]), 2);
    }

    #[test]
    fn test_hand_strenth_5() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            '4',
            '4',
            'Q',
            'K',
        ]), 4);
    }

    #[test]
    fn test_hand_strenth_6() {
        assert_eq!(get_hand_strength(&vec![
            'J',
            '4',
            '4',
            'Q',
            'Q',
        ]), 5);
    }
}
