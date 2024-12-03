#![feature(binary_heap_into_iter_sorted)]
use std::collections::{BinaryHeap, HashMap, HashSet};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::cmp::Reverse;


const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Read in the lists of numbers
        let mut left = BinaryHeap::new();
        let mut right = BinaryHeap::new();

        // Build left and right binary heaps by pushing each number into the heap on read
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let l = parts.next().ok_or(anyhow!("Invalid input"))?.parse::<i64>()?;
            let r = parts.next().ok_or(anyhow!("Invalid input"))?.parse::<i64>()?;

            left.push(Reverse(l));
            right.push(Reverse(r));
        }

        // Find the sum of the differences between the two heaps
        let answer = left.into_iter_sorted().zip(right.into_iter_sorted()).map(|(l, r)| {
            let l = l.0;
            let r = r.0;
            (l - r).abs() as usize
        }).sum::<usize>();


        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Calculate similarity scores between the two lists
        let mut left = Vec::new();
        let mut right = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let l = parts.next().ok_or(anyhow!("Invalid input"))?.parse::<i64>()?;
            let r = parts.next().ok_or(anyhow!("Invalid input"))?.parse::<i64>()?;

            left.push(l);

            // Basically computeIfAbsent
            let v = right.get_mut(&r);
            if let Some(v) = v {
                *v += 1;
            } else {
                right.insert(r, 1);
            }
        }

        let mut score = 0;

        for i in left.iter() {
            let count = right.get(&i).unwrap_or(&0);
            if count > &0 {
                score += i * count;
            }
        }

        Ok(score as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
