use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayThree {
    pub day: Day,
}

struct Mul {
    a: i64,
    b: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Enabled,
    Disabled,
}

impl DayThree {
    fn read_input(&self) -> Result<Vec<Mul>> {
        let mut multiplications = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

        for line in io::BufReader::new(file).lines() {
            let line = &line?;
            for (_, [a, b]) in re.captures_iter(line).map(|x| x.extract()) {
                multiplications.push(Mul {
                    a: a.parse::<i64>()?,
                    b: b.parse::<i64>()?,
                })
            }
        }
        Ok(multiplications)
    }

    fn read_input_for_second_task(&self) -> Result<Vec<Mul>> {
        let mut multiplications = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        let re = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d*),(\d*)\)").unwrap();

        let mut enabled = State::Enabled;

        for line in io::BufReader::new(file).lines() {
            let line = &line?;
            for captures in re.captures_iter(line) {
                match captures.get(0).expect("0th capture not null").as_str() {
                    "do()" => enabled = State::Enabled,
                    "don't()" => enabled = State::Disabled,
                    _ => match enabled {
                        State::Enabled => multiplications.push(Mul {
                            a: captures
                                .get(1)
                                .expect("1th capture not null")
                                .as_str()
                                .parse::<i64>()
                                .expect("parsing 1st captrue into i64"),
                            b: captures
                                .get(2)
                                .expect("2th capture not null")
                                .as_str()
                                .parse::<i64>()
                                .expect("parsing 2st captrue into i64"),
                        }),
                        State::Disabled => (),
                    },
                }
            }
        }

        Ok(multiplications)
    }
}

impl DayTrait<DayThree> for DayThree {
    fn new() -> DayThree {
        DayThree {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        return self
            .read_input()
            .expect("parse multiplications")
            .into_iter()
            .map(|x| x.a * x.b)
            .sum();
    }
    fn part_two(&self) -> i64 {
        return self
            .read_input_for_second_task()
            .expect("parse multiplications")
            .into_iter()
            .map(|x| x.a * x.b)
            .sum();
    }
}
