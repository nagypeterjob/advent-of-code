use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayNine {
    pub day: Day,
}

fn part2(representation: &mut Vec<Option<i64>>, mut id: i64) {
    id -= 1;
    let mut end = representation.len() as isize - 1;
    while end > 0 {
        let mut ptr = end as usize;
        while representation[ptr].is_none() || representation[ptr] != Some(id) {
            ptr -= 1;
        }
        let mut last = 0;
        let mut start_cache = 0;
        let mut end_cache = 0;

        while ptr >= end_cache && representation[ptr - end_cache] == Some(id) {
            end_cache += 1;
        }
        if id == 0 {
            break;
        }
        id -= 1;
        while start_cache < end_cache && last <= ptr {
            last += start_cache;
            start_cache = 0;
            while representation[last].is_some() {
                last += 1;
            }
            while last + start_cache <= ptr && representation[last + start_cache].is_none() {
                start_cache += 1;
            }
        }

        if start_cache >= end_cache {
            for i in 0..end_cache {
                representation.swap(last + i, ptr - i);
            }
        }
        end = (ptr - end_cache) as isize;
    }
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
            if i % 2 == 0 {
                for _ in 0..d {
                    representation.push(Some(id));
                }
                id += 1
            } else {
                for _ in 0..d {
                    representation.push(None);
                }
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

        let mut shadow2 = representation.clone();
        part2(&mut shadow2, id);

        (
            shadow
                .into_iter()
                .enumerate()
                .filter_map(|(i, d)| d.map(|x| x * i as i64))
                .sum(),
            shadow2
                .into_iter()
                .enumerate()
                .filter_map(|(i, d)| d.map(|x| x * i as i64))
                .sum(),
        )
    }
}
