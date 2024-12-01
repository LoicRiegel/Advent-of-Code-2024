use std::{fs::read_to_string, iter::zip};

const INPUT_FILE_NAME: &str = "input.txt";

fn read_input(filename: &str) -> Vec<(u32, u32)> {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    read_to_string(filename).unwrap().lines().for_each(|line| {
        let mut line_split = line.split("   ");
        let first_nb: u32 = line_split.next().unwrap().parse().unwrap();
        let second_nb: u32 = line_split.next().unwrap().parse().unwrap();
        first_list.push(first_nb);
        second_list.push(second_nb);
    });

    first_list.sort();
    second_list.sort();

    let mut two_lists: Vec<(u32, u32)> = Vec::new();
    zip(first_list, second_list).for_each(|entry| two_lists.push(entry));
    two_lists
}

fn total_distance(two_lists: Vec<(u32, u32)>) -> u64 {
    let mut total_distance: u64 = 0;
    two_lists.into_iter().for_each(|entry| {
        let diff = entry.0.abs_diff(entry.1);
        total_distance = total_distance + (diff as u64);
    });
    total_distance
}

fn main() {
    let two_lists = read_input(INPUT_FILE_NAME);
    dbg!(&two_lists);
    let total_distance = total_distance(two_lists);
    dbg!(total_distance);
}
