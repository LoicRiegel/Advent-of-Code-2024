use std::fs::read_to_string;
use std::iter::zip;

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

#[derive(Debug, PartialEq)]
enum LevelIncrease {
    Increasing,
    Decreasing,
}

trait Level {
    fn increasing(&self) -> Result<LevelIncrease, ()>;

    fn is_valid(&self) -> bool;
}

impl Level for Vec<u8> {
    fn increasing(&self) -> Result<LevelIncrease, ()> {
        let first = self.first().ok_or(())?;
        let second = self.get(1).ok_or(())?;
        if first < second {
            return Ok(LevelIncrease::Increasing);
        }
        Ok(LevelIncrease::Decreasing)
    }

    fn is_valid(&self) -> bool {
        let increase = match self.increasing() {
            Ok(level) => level,
            Err(_) => return false,
        };

        let first_slices = self.split_last().unwrap().1;
        let second_slices = self.split_first().unwrap().1;

        for nb_couple in zip(first_slices, second_slices) {
            if nb_couple.0.abs_diff(*nb_couple.1) > 3 {
                return false;
            }
            if *nb_couple.0 == *nb_couple.1
                || (*nb_couple.0 < *nb_couple.1 && increase == LevelIncrease::Decreasing)
                || (*nb_couple.0 > *nb_couple.1 && increase == LevelIncrease::Increasing)
            {
                return false;
            }
        }

        true
    }
}

fn main() {
    let reports = read_input(INPUT_FILE_NAME);
    dbg!(&reports);

    let valid_reports: Vec<Vec<u8>> = reports
        .clone()
        .into_iter()
        .filter(|entry| entry.is_valid())
        .collect();
    dbg!(&valid_reports);
    dbg!(&reports.len());
    dbg!(&valid_reports.len());
}
