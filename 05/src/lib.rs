use regex::Regex;
use std::{collections::HashMap, ops::RangeInclusive, ops::Range};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapItem {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Map {
    from: MapItem,
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64
}

#[derive(Debug, Clone)]
struct MapEntry {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64
}


fn get_map_item(map_item_str: &str) -> Result<MapItem, String> {
    match map_item_str {
        "seed" => return Ok(MapItem::Seed),
        "soil" => return Ok(MapItem::Soil),
        "fertilizer" => return Ok(MapItem::Fertilizer),
        "water" => return Ok(MapItem::Water),
        "light" => return Ok(MapItem::Light),
        "temperature" => return Ok(MapItem::Temperature),
        "humidity" => return Ok(MapItem::Humidity),
        "location" => return Ok(MapItem::Location),
        _ => return Err(format!("could not match {} to MapItem", map_item_str)),
    }
}

fn get_map_data(input: &str) -> Vec<Map> {
    let from_to_regex = Regex::new(r"(?<from>[a-z]+)-to-(?<to>[a-z]+) map:").unwrap();
    let map_entry_regex = Regex::new(r"(?<destination_range_start>\d+) (?<source_range_start>\d+) (?<range_length>\d+)").unwrap();

    let mut vec_vec_map_entries: Vec<Map> = vec![];
    let mut from = MapItem::Seed;
    for line in input.lines().skip(2) {
        if let Some(map_title_match) = from_to_regex.captures(line) {
            let from_str: &str = map_title_match["from"].into();
            from = get_map_item(from_str).unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });
        } else if let Some(map_entry_match) = map_entry_regex.captures(line) {
            let destination_range_start_str: &str = map_entry_match["destination_range_start"].into();
            let source_range_start_str: &str = map_entry_match["source_range_start"].into();
            let range_length_str: &str = map_entry_match["range_length"].into();

            let destination_range_start: u64 = destination_range_start_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });
            let source_range_start: u64 = source_range_start_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });
            let range_length: u64 = range_length_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });

            vec_vec_map_entries.push(Map { from: from.clone(), destination_range_start, source_range_start, range_length })
        }
        else {
            if line.len() != 0 {
                println!("So i didn't process this line: {:?}", line);
            }
        }
    }
    return vec_vec_map_entries;
}

fn get_seeds(seeds_line: &str) -> Vec<u64> {
    let number_regex = Regex::new(r"\b(\d+)\b").unwrap();
    return number_regex
        .captures_iter(seeds_line)
        .flat_map(|captures| captures.get(1))
        .map(|number_str| number_str.as_str().parse::<u64>())
        .filter_map(Result::ok)
        .collect();
}

fn get_map_item_to_maps(vec_vec_map_entries: &Vec<Map>) -> HashMap::<MapItem, HashMap<RangeInclusive<u64>, u64>> {
    let mut map_item_to_maps = HashMap::<MapItem, HashMap<RangeInclusive<u64>, u64>>::new();

    for map_entry in vec_vec_map_entries.iter() {
        if !map_item_to_maps.contains_key(&map_entry.from) {
            map_item_to_maps.insert(map_entry.from.clone(), HashMap::new());
        }
        let Some(map_of_ranges_to_dest) = map_item_to_maps.get_mut(&map_entry.from) else {
            panic!("No vector found for {:?}", map_entry.from);
        };
        let source_range_start = map_entry.source_range_start.clone();
        let range_length = map_entry.range_length.clone();
        let destination_range_start = map_entry.destination_range_start.clone();

        map_of_ranges_to_dest.insert(
            source_range_start..=(source_range_start + range_length - 1),
            destination_range_start
        );
    }

    return map_item_to_maps;
}

fn get_next_map_item(map_item: &MapItem) -> Option<MapItem> {
    match map_item {
        MapItem::Seed => return Some(MapItem::Soil),
        MapItem::Soil => return Some(MapItem::Fertilizer),
        MapItem::Fertilizer => return Some(MapItem::Water),
        MapItem::Water => return Some(MapItem::Light),
        MapItem::Light => return Some(MapItem::Temperature),
        MapItem::Temperature => return Some(MapItem::Humidity),
        MapItem::Humidity => return Some(MapItem::Location),
        MapItem::Location => return None,

    }
}

fn get_location_value(seed: u64, map_item_to_maps: &HashMap::<MapItem, HashMap<RangeInclusive<u64>, u64>>) -> u64 {
    let mut current_map_item = MapItem::Seed;
    let mut current_value = seed;
    while current_map_item != MapItem::Location {
        let map_for_range_to_destination_value = map_item_to_maps.get(&current_map_item)
            .unwrap_or_else(|| panic!("Vec of maps not found for {:?}", current_map_item));
        let has_map: bool= map_for_range_to_destination_value
            .keys()
            .any(|key| {
                return key.contains(&current_value);
            });

        if has_map {
            let (range, destination) = map_for_range_to_destination_value
                .iter()
                .filter(|(range, _)| range.contains(&current_value))
                .next()
                .unwrap_or_else(|| panic!("Could not get the entry with current value in the range"));
            let offset = current_value - range.start();
            current_value = destination + offset;
        }

        current_map_item = get_next_map_item(&current_map_item).unwrap_or_else(|| panic!("Location maps to None"));
    }
    return current_value;
}

fn get_min_location(input: &str, seeds: Vec<u64>) -> u64 {
    let vec_vec_map_entries: Vec<Map> = get_map_data(input);
    let map_item_to_maps = get_map_item_to_maps(&vec_vec_map_entries);

    return seeds
        .into_iter()
        .map(|seed| get_location_value(seed, &map_item_to_maps))
        .min()
        .unwrap_or_else(|| panic!("could not unwrap min"));
}

fn get_seeds_line(input: &str) -> &str {
    let Some(seeds_line) = input.lines().next() else {
        panic!("Cannot get seeds line");
    };
    return seeds_line;
}

pub fn part1(input: &str) -> u64 {
    let seeds: Vec<u64> = get_seeds(get_seeds_line(input));
    return get_min_location(input, seeds);
}

fn get_seeds_for_input_as_range(seeds_line: &str) -> Vec<Range<u64>> {
    let all_seed_values = get_seeds(seeds_line);
    let seed_start_range_length_pairs: Vec<(u64, u64)> = all_seed_values
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
    return seed_start_range_length_pairs
        .iter()
        .map(|(start, length)| (*start..(*start+*length)))
        .collect();
}

fn get_range_maps(input: &str) -> Vec<Vec<MapEntry>> {
    let from_to_regex = Regex::new(r"(?<from>[a-z]+)-to-(?<to>[a-z]+) map:").unwrap();
    let map_entry_regex = Regex::new(r"(?<destination_range_start>\d+) (?<source_range_start>\d+) (?<range_length>\d+)").unwrap();

    let mut vec_vec_map_entries: Vec<Vec<MapEntry>> = vec![];
    let mut vec_of_map_entries: Vec<MapEntry> = vec![];
    for line in input.lines().skip(2) {
        if let Some(_) = from_to_regex.captures(line) {
            vec_of_map_entries = vec![];
        } else if let Some(map_entry_match) = map_entry_regex.captures(line) {
            let destination_range_start_str: &str = map_entry_match["destination_range_start"].into();
            let source_range_start_str: &str = map_entry_match["source_range_start"].into();
            let range_length_str: &str = map_entry_match["range_length"].into();

            let destination_range_start: u64 = destination_range_start_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });
            let source_range_start: u64 = source_range_start_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });
            let range_length: u64 = range_length_str.parse().unwrap_or_else(|err| {
                panic!("Error:{}", err);
            });

            vec_of_map_entries.push(MapEntry { destination_range_start, source_range_start, range_length });
        }
        else {
            if line.len() != 0 {
                println!("So i didn't process this line: {:?}", line);
            }
            vec_vec_map_entries.push(vec_of_map_entries.clone());
        }
    }
    vec_vec_map_entries.push(vec_of_map_entries);
    return vec_vec_map_entries;
}

pub fn part2(input: &str) -> u64 {
    let mut input_ranges: Vec<Range<u64>> = get_seeds_for_input_as_range(get_seeds_line(input));
    let blocks: Vec<Vec<MapEntry>> = get_range_maps(input);

    for map_entries in blocks.into_iter() {
        let mut mapped_input_ranges: Vec<Range<u64>> = vec![];
        while input_ranges.len() != 0 {
            let mut has_overlap = false;
            let curr_input_range = input_ranges.remove(0);
            for map_entry in map_entries.iter() {
                let overlap_start = std::cmp::max(curr_input_range.start, map_entry.source_range_start);
                let overlap_end = std::cmp::min(
                    curr_input_range.end, map_entry.source_range_start + map_entry.range_length
                );

                if overlap_start < overlap_end {
                    has_overlap = true;
                    let mapped_range_start: u64 = overlap_start - map_entry.source_range_start +
                        map_entry.destination_range_start;
                    let mapped_range_end: u64 = overlap_end - map_entry.source_range_start +
                        map_entry.destination_range_start;
                    mapped_input_ranges.push(mapped_range_start..mapped_range_end);

                    // push the ranges not in the overlap from the current range back into
                    // input_ranges
                    if curr_input_range.start < overlap_start {
                        input_ranges.push(curr_input_range.start..overlap_start);
                    }
                    if curr_input_range.end > overlap_end {
                        input_ranges.push(overlap_end..curr_input_range.end);
                    }
                    break;
                }
            }
            if !has_overlap {
                mapped_input_ranges.push(curr_input_range);
            }
        }
        input_ranges = mapped_input_ranges;
    }

    return input_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap_or_else(|| panic!(""));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part1(input), 35);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../input.txt");

        assert_eq!(part2(input), 46);
    }
}
