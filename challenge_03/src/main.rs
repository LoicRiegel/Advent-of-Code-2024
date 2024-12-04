use regex::Regex;
use std::fs::read_to_string;

const INPUT_FILE_NAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

fn main() {
    let text = read_input(INPUT_FILE_NAME);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex!");

    let numbers: Vec<(u32, u32)> = re.captures_iter(&text).fold(vec![], |mut acc, cap| {
        acc.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
        acc
    });

    let total: u32 = numbers.iter().fold(0, |acc, e| acc + (e.0 * e.1));
    dbg!(&total);
}
