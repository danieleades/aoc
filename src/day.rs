pub trait Solution: Fn(&str) -> u32 + Sized + 'static {}

impl<F> Solution for F where F: Fn(&str) -> u32 + Sized + 'static {}

pub struct Day {
    test_input: &'static str,
    input: &'static str,
    part1: Option<Part>,
    part2: Option<Part>,
}

pub struct Part {
    test_expected: u32,
    solution: &'static dyn Fn(&str) -> u32,
}

impl Part {
    fn run(&self, input: &str) -> u32 {
        (self.solution)(input)
    }
}

pub struct Builder {
    day: Day,
}

impl Builder {
    pub const fn part1<F>(mut self, test_expected: u32, solution: &'static F) -> Self
    where
        F: Solution,
    {
        let part = Part {
            test_expected,
            solution,
        };
        self.day.part1 = Some(part);
        self
    }

    pub const fn part2<F>(mut self, test_expected: u32, solution: &'static F) -> Self
    where
        F: Solution,
    {
        let part = Part {
            test_expected,
            solution,
        };
        self.day.part2 = Some(part);
        self
    }

    pub const fn build(self) -> Day {
        self.day
    }
}

impl Day {
    pub const fn builder(test_input: &'static str, input: &'static str) -> Builder {
        let day = Day {
            test_input,
            input,
            part1: None,
            part2: None,
        };
        Builder { day }
    }
    pub fn run(&self, day: u32) {
        match &self.part1 {
            Some(part) => {
                println!("day {}, part 1: {}", day, part.run(self.input));
            }
            None => println!("no solution registered for day {}, part 1", day),
        }
        match &self.part2 {
            Some(part) => {
                println!("day {}, part 2: {}", day, part.run(self.input));
            }
            None => println!("no solution registered for day {}, part 1", day),
        }
    }
}
