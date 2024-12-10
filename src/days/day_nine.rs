use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayNine {
    pub day: Day,
}

impl DayNine {
    fn read_input(&self) -> Result<Vec<char>> {
        let mut chars: Vec<char> = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            chars = line.chars().collect();
        }
        Ok(chars)
    }
}

impl DayTrait<DayNine> for DayNine {
    fn new() -> DayNine {
        DayNine {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        0
    }
    fn part_two(&self) -> i64 {
        0
    }
    fn solution(&self) -> (i64, i64) {
        let input = self.read_input().expect("read input");
        let mut representation = vec![];

        let mut id: i64 = 0;
        for (i, c) in input.iter().enumerate() {
            let d = c.to_digit(10).expect("parsing char") as i64;
            for _ in 0..d {
                if i % 2 == 0 {
                    representation.push(Some(id));
                } else {
                    representation.push(None);
                }
            }
            if i % 2 == 0 {
                id += 1;
            }
        }

        let mut shadow = representation.clone();
        let mut i = 0;
        let mut j = representation.len() - 1;
        while i < j {
            while representation[i].is_some() {
                i += 1;
            }
            while representation[j].is_none() {
                j -= 1;
            }
            if i < j {
                shadow.swap(i, j);
            }
            i += 1;
            j -= 1;
        }
        (
            shadow
                .into_iter()
                .enumerate()
                .filter_map(|(i, d)| d.map(|x| x * i as i64))
                .sum(),
            0,
        )
    }
}
