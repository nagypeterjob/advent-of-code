use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use std::fs::File;
use std::io::{self, BufRead};

pub struct DayOne {
    pub day: Day,
}

impl DayOne {
    fn read_input(&self) -> Result<(Vec<i64>, Vec<i64>)> {
        let mut a = vec![];
        let mut b = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let values: Vec<&str> = line.split("   ").collect();
            a.push(values[0].parse::<i64>()?);
            b.push(values[1].parse::<i64>()?);
        }
        Ok((a, b))
    }
}

impl DayTrait<DayOne> for DayOne {
    fn new() -> DayOne {
        DayOne {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        let (mut list_a, mut list_b) = self.read_input().expect("read input form file");
        list_a.sort();
        list_b.sort();

        return list_a
            .iter()
            .zip(list_b.iter())
            .into_iter()
            .map(|(a, b)| (a - b).abs())
            .sum();
    }
    fn part_two(&self) -> i64 {
        let (list_a, list_b) = self.read_input().expect("read input form file");

        return list_a
            .into_iter()
            .map(|x| x * list_b.to_owned().into_iter().filter(|&e| e == x).count() as i64)
            .sum();
    }
}
