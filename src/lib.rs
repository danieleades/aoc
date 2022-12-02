mod day;
use std::collections::BTreeMap;

pub use day::Day;

pub struct Runner {
    days: BTreeMap<u32, Day>,
}

impl Runner {
    pub fn new(days: impl Into<BTreeMap<u32, Day>>) -> Self {
        Self { days: days.into() }
    }

    pub fn run(&self) {
        for (day, solution) in &self.days {
            solution.run(*day);
        }
    }
}
