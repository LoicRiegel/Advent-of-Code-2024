use counter::Counter;
use std::fs::read_to_string;
use std::iter::zip;

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

fn total_distance(two_lists: &[(u32, u32)]) -> u64 {
    let mut total_distance: u64 = 0;
    two_lists.iter().for_each(|entry| {
        let diff = entry.0.abs_diff(entry.1);
        total_distance += diff as u64;
    });
    total_distance
}

fn similarity_score(two_lists: &[(u32, u32)]) -> usize {
    let mut similqrity_score: usize = 0;

    let second_list_counter: Counter<u32> = two_lists
        .iter()
        .map(|entry| entry.1)
        .collect::<Counter<u32>>();

    two_lists.iter().map(|entry| entry.0).for_each(|first_nb| {
        let occurrences_in_second_list = second_list_counter.get(&first_nb).unwrap_or(&0);
        similqrity_score += (first_nb as usize) * occurrences_in_second_list;
    });

    similqrity_score
}

fn main() {
    let two_lists = read_input(INPUT_FILE_NAME);
    //dbg!(&two_lists);
    let total_distance = total_distance(&two_lists);
    dbg!(total_distance);
    let similarity_score = similarity_score(&two_lists);
    dbg!(similarity_score);
}
