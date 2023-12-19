use::std::collections::HashMap;
use::num::integer::lcm;
use::regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn get_instructions(input: &str) -> Vec<char> {
   let first_line: String = input.lines().take(1).collect();
    return first_line.chars().collect();
}

fn get_node_map(input: &str) -> HashMap<String, Node> {
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let node_regex = Regex::new(
        r"(?<node_name>[A-Z]{3,3}) = \((?<left_node>[A-Z]{3,3}), (?<right_node>[A-Z]{3,3})\)"
    ).unwrap();

    for line in input.lines().skip(2) {
        if let Some(line_cap) = node_regex.captures(line) {
            let node_name = line_cap["node_name"].to_string();
            let left = line_cap["left_node"].to_string();
            let right = line_cap["right_node"].to_string();
            node_map.insert(node_name, Node {left, right});
        } else {
            panic!("paniced at line: {:?}", line);
        }
    }

    return node_map;
}

fn get_num_steps_to_reach_zzz(instructions: Vec<char>, node_map: HashMap<String, Node>) -> u32 {
    let mut i = 0;
    let mut num_steps = 0;
    let mut current_node = "AAA".to_string();
    while current_node != "ZZZ" {
        let direction = instructions[i];
        let Some(node) = node_map.get(&current_node) else {
            panic!("{:?} not in map", current_node);
        };
        
        if direction == 'L' {
            current_node = node.left.clone();
        } else if direction == 'R' {
            current_node = node.right.clone();
        } else {
            panic!("Direction is not L or R, it is: {:?}", direction);
        }
        i = (i + 1) % instructions.len();
        num_steps += 1;
    }
    return num_steps;
}
fn get_num_steps_to_reach_zzz_with_node(instructions: &Vec<char>, node_map: &HashMap<String, Node>, node: &String) -> u64 {
    let mut i = 0;
    let mut num_steps: u64 = 0;
    let mut current_node = node.clone();
    while current_node.get(2..3).unwrap() != "Z" {
        let direction = instructions[i];
        let Some(node) = node_map.get(&current_node) else {
            panic!("{:?} not in map", current_node);
        };
        
        if direction == 'L' {
            current_node = node.left.clone();
        } else if direction == 'R' {
            current_node = node.right.clone();
        } else {
            panic!("Direction is not L or R, it is: {:?}", direction);
        }
        i = (i + 1) % instructions.len();
        num_steps += 1;
    }
    return num_steps;
}

fn get_num_steps_for_all_starting_nodes_to_reach_zzz(instructions: Vec<char>, node_map: HashMap<String, Node>) -> u32 {
    let mut i = 0;
    let mut num_steps = 0;

    let mut current_nodes: Vec<String> = node_map
        .keys()
        .filter(|key| key.get(2..3).unwrap() == "A")
        .map(|key| key.to_string())
        .collect();

    println!("{:?}", current_nodes);

    while !current_nodes.iter().all(|node| node.get(2..3).unwrap() == "Z")  {
        let direction = instructions[i];
        for j in 0..current_nodes.len() {
            let Some(node) = node_map.get(&current_nodes[j]) else {
                panic!("{:?} not in map", current_nodes[j]);
            };

            if direction == 'L' {
                current_nodes[j] = node.left.clone();
            } else if direction == 'R' {
                current_nodes[j] = node.right.clone();
            } else {
                panic!("Direction is not L or R, it is: {:?}", direction);
            }
        }
        i = (i + 1) % instructions.len();
        num_steps += 1;
    }
    return num_steps;
}

pub fn part1(input: &str) -> u32 {
    let instructions = get_instructions(input);
    let node_map = get_node_map(input);
    return get_num_steps_to_reach_zzz(instructions, node_map);
}

pub fn part2(input: &str) -> u64{
    let instructions = get_instructions(input);
    let node_map = get_node_map(input);
    let current_nodes: Vec<String> = node_map
        .keys()
        .filter(|key| key.get(2..3).unwrap() == "A")
        .map(|key| key.to_string())
        .collect();
     let num_steps_to_reach_z: Vec<u64> = current_nodes
        .iter()
        .map(|node| get_num_steps_to_reach_zzz_with_node(&instructions, &node_map, node))
        .collect();
    return find_lcm(num_steps_to_reach_z);
}
fn find_lcm(numbers: Vec<u64>) -> u64 {
    if numbers.is_empty() {
        return 0; // LCM is undefined for an empty list
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = num::integer::lcm(result, num);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part1(input), 0)
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../test_input.txt");

        assert_eq!(part2(input), 0);
    }
}
