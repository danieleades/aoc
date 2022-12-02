use std::{num::ParseIntError, str::FromStr};

use advent_of_code::Day;

struct Pack {
    calories: Vec<u32>,
}

impl Pack {
    fn total(&self) -> u32 {
        self.calories.iter().fold(0, |mut acc, elem| {
            acc += elem;
            acc
        })
    }
}

impl FromStr for Pack {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<_, _> = s.split('\n').map(u32::from_str).collect();
        Ok(Self { calories: result? })
    }
}

mod part1 {
    use std::str::FromStr;

    use super::Pack;

    pub const TEST_EXPECTED: u32 = 24000;

    pub fn solution(input: &str) -> u32 {
        let result: Result<_, _> = input.split("\n\n").map(Pack::from_str).collect();
        let packs: Vec<Pack> = result.unwrap();
        packs.iter().map(Pack::total).max().unwrap()
    }
}

mod part2 {
    use std::str::FromStr;

    use crate::Pack;

    pub const TEST_EXPECTED: u32 = 45000;

    pub fn solution(input: &str) -> u32 {
        let result: Result<Vec<Pack>, _> = input.split("\n\n").map(Pack::from_str).collect();
        let mut totals: Vec<_> = result.unwrap().iter().map(Pack::total).collect();
        totals.sort();
        totals.iter().rev().take(3).sum()
    }
}

pub const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

const INPUT: &str = include_str!("input/2022-01");

fn solution_part1() {
    let test_result = part1::solution(TEST_INPUT);

    println!("test result: {}", test_result);
    assert_eq!(test_result, part1::TEST_EXPECTED);

    println!("result: {}", part1::solution(INPUT));
}

fn solution_part2() {
    let test_result = part2::solution(TEST_INPUT);

    println!("test result: {}", test_result);
    assert_eq!(test_result, part2::TEST_EXPECTED);

    println!("result: {}", part2::solution(INPUT));
}

fn main() {
    let day = Day::builder(TEST_INPUT, INPUT)
        .part1(part1::TEST_EXPECTED, part1::solution)
        .part2(part2::TEST_EXPECTED, part2::solution)
        .build();
    day.run(1);
}
