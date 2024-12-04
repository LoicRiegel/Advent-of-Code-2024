use std::fs::read_to_string;

const INPUT_FILE_NAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

fn main() {
    let text =  read_input(INPUT_FILE_NAME);
    dbg!(text);
}