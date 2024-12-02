use challenge_02::Level;
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

    let valid_reports: Vec<Vec<u8>> = reports
        .clone()
        .into_iter()
        .filter(|entry| entry.is_valid())
        .collect();
    let valid_reports_with_problem_dampener: Vec<Vec<u8>> = reports
        .clone()
        .into_iter()
        .filter(|entry| entry.is_valid_with_problem_dampener())
        .collect();

    // dbg!(&reports);
    // dbg!(&valid_reports);
    // dbg!(&valid_reports_with_problem_dampener);
    dbg!(&reports.len());
    dbg!(&valid_reports.len());
    dbg!(&valid_reports_with_problem_dampener.len());
}
