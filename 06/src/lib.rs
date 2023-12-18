use core::time;

use regex::Regex;

#[derive(Debug)]
struct RaceRecord {
    distance: u64,
    time: u64,
}

fn get_race_records(input: &str) -> Vec<RaceRecord> {
    let lines: Vec<&str> = input.lines().collect();
    let number_regex = Regex::new(r"\b(\d+)\b").unwrap();
    let times: Vec<u64> = number_regex
        .captures_iter(lines[0])
        .flat_map(|captures| captures.get(1))
        .map(|number_str| number_str.as_str().parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    let distances: Vec<u64> = number_regex
        .captures_iter(lines[1])
        .flat_map(|captures| captures.get(1))
        .map(|number_str| number_str.as_str().parse::<u64>())
        .filter_map(Result::ok)
        .collect();

    let mut race_records: Vec<RaceRecord> = vec![];
    for i in 0..times.len() {
        race_records.push(RaceRecord { distance: distances[i], time: times[i] });
    }

    return race_records;
}

fn get_race_record(input: &str) -> RaceRecord {
    let lines: Vec<&str> = input.lines().collect();
    let number_regex = Regex::new(r"\b(\d+)\b").unwrap();
    let time_string: String = number_regex
        .captures_iter(lines[0])
        .flat_map(|captures| captures.get(1))
        .map(|number_str| number_str.as_str())
        .collect();
    let distance_string: String = number_regex
        .captures_iter(lines[1])
        .flat_map(|captures| captures.get(1))
        .map(|number_str| number_str.as_str())
        .collect();

    return RaceRecord {
        distance: distance_string.parse::<u64>().unwrap_or_else(|err| panic!("{:?}", err)),
        time: time_string.parse::<u64>().unwrap_or_else(|err| panic!("{:?}", err))
    }
}

fn get_boat_distance(hold_time: &u64, time_given: &u64) -> u64 {
    if hold_time >= time_given {
        return 0;
    }

    return hold_time * (time_given - hold_time);
}

fn get_num_ways_to_beat_record(record_distance: &u64, time_given: &u64) -> u64 {
    let mut i = 1;
    let mut j = time_given - 1;
    while get_boat_distance(&i, time_given) <= *record_distance {
        i += 1;
    }

    while get_boat_distance(&j, time_given) <= *record_distance {
        j -= 1;
    }

    return j - i + 1;
}

pub fn part1(input: &str) -> u64 {
    let race_records = get_race_records(input);
    return race_records
        .iter()
        .map(|race_record| get_num_ways_to_beat_record(&race_record.distance, &race_record.time))
        .product();
}

pub fn part2(input: &str) -> u64 {
    let race_record = get_race_record(input);

    return get_num_ways_to_beat_record(&race_record.distance, &race_record.time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part1(input), 288)
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part2(input), 71503);
    }
}
