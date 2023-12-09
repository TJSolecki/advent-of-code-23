use std::fs;   

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read in the file");
    let numbers: Vec<_> = input.lines().map(|line| {
        line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>()
    }).collect();
    let first_and_last_numbers: Vec<_> = numbers.into_iter().map(|x| [x[0], x[x.len() - 1]]).collect();
    let line_numbers: Vec<i32> = first_and_last_numbers.into_iter().map(|x| x.iter().collect::<String>().parse::<i32>().unwrap_or(0)).collect();
    let total_sum: i32 = line_numbers.iter().fold(0, |acc, x| acc + x);
    println!("{:?}", total_sum);
}
