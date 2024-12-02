use std::fs::read_to_string;

const INPUT_FILE_NAME: &str = "input.txt";

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    let mut reports: Vec<Vec<u8>> = Vec::new();

    read_to_string(filename).unwrap().lines().for_each(|line| {
        let line_split = line.split(" ");

        let levels: Vec<u8> = line_split.fold(Vec::<u8>::new(), |mut acc, part| {
            acc.push(part.parse::<u8>().unwrap());
            acc
        });
        reports.push(levels);
    });

    reports
}

fn main() {
    let reports = read_input(INPUT_FILE_NAME);
    dbg!(&reports);
}
