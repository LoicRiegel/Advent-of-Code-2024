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

    fn is_valid_with_problem_dampener(&self) -> bool;
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

    fn is_valid_with_problem_dampener(&self) -> bool {
        if self.is_valid() {
            return true;
        }
        let sub_vectors_valid: Vec<bool> = self.iter().enumerate().fold(
            Vec::default(), |mut acc, (idx, _)| {
                let mut sub_vector = self.clone();
                sub_vector.remove(idx);
                acc.push(sub_vector.is_valid());
                acc
            }
        );
    
        sub_vectors_valid.contains(&true)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3, 2, 1], true)]
    #[case(vec![1, 2, 3], true)]
    #[case(vec![1, 2, 0, 3], false)]
    #[case(vec![1, 2, 2], false)]
    #[case(vec![1, 2, 1], false)]
    #[case(vec![2, 1, 2], false)]
    fn test_is_valid(#[case] input: Vec<u8>, #[case] expected: bool) {
        assert_eq!(input.is_valid(), expected);
    }

    #[rstest]
    #[case(vec![1, 2, 9], true)]
    #[case(vec![1, 2, 9, 10], false)]
    fn test_is_valid_with_problem_dampener(#[case] input: Vec<u8>, #[case] expected: bool) {
        assert_eq!(input.is_valid_with_problem_dampener(), expected);
    }
}