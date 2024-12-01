use std::fs::read_to_string;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Debug, Default)]
struct Input {
    first_list: Vec<u32>,
    second_list: Vec<u32>,
}

fn read_input(filename: &str) -> Input {
    let mut input = Input::default();

    read_to_string(filename).unwrap().lines().for_each(|line| {
        let mut line_split = line.split("   ");
        let first_nb: u32 = line_split.next().unwrap().parse().unwrap();
        let second_nb: u32 = line_split.next().unwrap().parse().unwrap();
        input.first_list.push(first_nb);
        input.second_list.push(second_nb);
    });

    input
}

fn main() {
    let input = read_input(INPUT_FILE_NAME);
    dbg!(&input);
}
