use advent_of_code::Runner;

mod day1 {
    use std::{num::ParseIntError, str::FromStr};

    use advent_of_code::Day;

    pub const SOLUTION: Day = Day::builder(TEST_INPUT, INPUT)
        .part1(part1::TEST_EXPECTED, &part1::solution)
        .part2(part2::TEST_EXPECTED, &part2::solution)
        .build();

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

    pub mod part1 {
        use std::str::FromStr;

        use super::Pack;

        pub const TEST_EXPECTED: u32 = 24000;

        pub fn solution(input: &str) -> u32 {
            let result: Result<_, _> = input.split("\n\n").map(Pack::from_str).collect();
            let packs: Vec<Pack> = result.unwrap();
            packs.iter().map(Pack::total).max().unwrap()
        }
    }

    pub mod part2 {
        use std::str::FromStr;

        use crate::day1::Pack;

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

    pub const INPUT: &str = include_str!("input/2022-01");
}

fn main() {
    let runner = Runner::new([(1, day1::SOLUTION)]);
    runner.run();
}
