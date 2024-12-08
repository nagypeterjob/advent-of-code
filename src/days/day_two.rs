use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use std::fs::File;
use std::io::{self, BufRead};

pub struct DayTwo {
    pub day: Day,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Initial,
    Asc,
    Desc,
    Invalid,
}

impl DayTwo {
    fn read_input(&self) -> Result<Vec<Vec<i64>>> {
        let mut outer = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let mut inner = vec![];

            let values: Vec<&str> = line.split(' ').collect();
            for i in values {
                inner.push(i.parse::<i64>()?);
            }
            outer.push(inner)
        }
        Ok(outer)
    }
}

impl DayTrait<DayTwo> for DayTwo {
    fn new() -> DayTwo {
        DayTwo {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn solution(&self) -> (i64, i64) {
        (0, 0)
    }
    fn part_one(&self) -> i64 {
        let input = self.read_input().expect("construct nested loop");

        let mut safe = 0;
        for i in 0..input.len() {
            if is_safe(input[i].to_owned()) {
                safe += 1;
            }
        }

        safe
    }
    fn part_two(&self) -> i64 {
        let input = self.read_input().expect("construct nested loop");

        let mut safe = 0;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let mut cl = input[i].clone();
                cl.remove(j);
                if is_safe(cl) {
                    safe += 1;
                    break;
                }
            }
        }

        safe
    }
}

fn get_direction(a: i64, b: i64) -> Direction {
    if a < b {
        return Direction::Asc;
    }

    if b < a {
        return Direction::Desc;
    }

    return Direction::Invalid;
}

fn validate_direction(prev: Direction, cur: Direction) -> bool {
    cur != Direction::Invalid
        && ((prev == Direction::Initial)
            || (prev == Direction::Asc && cur == Direction::Asc)
            || (prev == Direction::Desc && cur == Direction::Desc))
}

fn validate_increase(a: i64, b: i64) -> bool {
    let diff = (a - b).abs();
    return diff >= 1 && diff <= 3;
}

fn is_safe(levels: Vec<i64>) -> bool {
    let mut direction = Direction::Initial;
    let mut safe = 0;
    for i in 1..levels.len() {
        let prev = levels[i - 1];
        let cur = levels[i];

        let current_direction = get_direction(prev, cur);
        if !validate_direction(direction, current_direction.clone())
            || !validate_increase(prev, cur)
        {
            break;
        }
        direction = current_direction;
        safe += 1;
    }

    safe == levels.len() - 1
}
