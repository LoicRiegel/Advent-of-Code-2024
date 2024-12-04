use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT_FILE_NAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

fn total_part_1(text: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex!");

    let numbers: Vec<(u32, u32)> = re.captures_iter(text).fold(vec![], |mut acc, cap| {
        acc.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
        acc
    });

    numbers.iter().fold(0, |acc, e| acc + (e.0 * e.1))
}

fn total_part_2(text: &str) -> u32 {
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex!");

    let re_do = Regex::new(r"do\(\)").expect("Invalid regex!");
    let re_dont = Regex::new(r"don't\(\)").expect("Invalid regex!");

    let mut okay_table: HashMap<usize, bool> = HashMap::new();
    okay_table.insert(0, true);

    for cap in re_do.captures_iter(text) {
        okay_table.insert(cap.get(0).unwrap().end(), true);
    }

    for cap in re_dont.captures_iter(text) {
        okay_table.insert(cap.get(0).unwrap().end(), false);
    }

    let mut numbers: Vec<(u32, u32)> = vec![];
    let mut keys: Vec<usize> = okay_table.keys().cloned().collect();
    keys.sort();
    keys.reverse();

    for cap in re_mul.captures_iter(text) {
        let end = cap.get(0).unwrap().end();

        // check if should be added or not
        for key in keys.iter() {
            if key < &end {
                if *okay_table.get(key).unwrap() == true {
                    numbers.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
                }
                break;
            }
        }
    }

    numbers.iter().fold(0, |acc, e| acc + (e.0 * e.1))
}

fn main() {
    let text = read_input(INPUT_FILE_NAME);
    dbg!(total_part_1(&text));
    dbg!(total_part_2(&text));
}
