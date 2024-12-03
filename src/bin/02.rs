use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use thiserror::Error;
use adv_code_2024::*;
use crate::LevelError::UnsafeLevel;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[derive(Error, Debug)]
pub enum LevelError {
    #[error("Unsafe level at index {cur} and {next}")]
    UnsafeLevel { cur: usize, next: usize },
}

type LevelResult = Result<(), LevelError>;


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");


    // A safe level is monotonically increasing or decreasing AND increase by at least 1 but not more than 3 between each number
    // When we are unsafe, we return an error with the unsafe index
    fn is_safe_level(level: &[i64]) -> LevelResult {
        let mut cur = level[0];
        let mut next = level[1];

        // It has to monotonically increase or decrease
        let direction = (next - cur).signum();

        if direction == 0 {
            return Err(UnsafeLevel { cur: 0, next: 1 })
        }

        for i in 1..level.len() {
            cur = level[i -1];
            next = level[i];
            let diff = next - cur;
            let dir = diff.signum();
            let change = diff.abs();

            // Must maintain the same direction
            if dir != direction {
                return Err(UnsafeLevel { cur: i - 1, next: i })
            }

            // Must change by at least 1 but not more than 3
            if (change < 1) || (change > 3) {
                return Err(UnsafeLevel { cur: i - 1, next: i })
            }
        }

        LevelResult::Ok(())
    }


    // Calculate the number of safe and unsafe levels
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // We can process each row independently
        let mut safe: usize = 0;

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let mut level = Vec::new();
            for part in parts {
                level.push(part.parse::<i64>()?);
            }

            match is_safe_level(&level) {
                LevelResult::Ok(_) => {
                    // println!("{:?} was safe", level);
                    safe += 1;
                }
                Err(_) => {}
            }
        }

        Ok(safe)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");


    fn copy_without_index(level: &[i64], index: usize) -> Vec<i64> {
        let mut copy = Vec::new();
        for i in 0..level.len() {
            if i != index {
                copy.push(level[i]);
            }
        }
        copy
    }


    // Similar to part 1 but also allows safely if there is one anomaly
    fn is_mostly_safe(levels: &[i64]) -> LevelResult {
        for i in 0..levels.len() {
            let copy = copy_without_index(levels, i);
            match is_safe_level(&copy) {
                LevelResult::Ok(_) => {
                    return LevelResult::Ok(());
                }
                Err(_) => {}
            }
        }

        Err(UnsafeLevel { cur: 0, next: 1 })
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // We can process each row independently
        let mut safe: usize = 0;

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let mut level = Vec::new();
            for part in parts {
                level.push(part.parse::<i64>()?);
            }

            match is_mostly_safe(&level) {
                LevelResult::Ok(_) => {
                    // println!("{:?} was safe", level);
                    safe += 1;
                }
                Err(_) => {}
            }
        }
        Ok(safe)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
