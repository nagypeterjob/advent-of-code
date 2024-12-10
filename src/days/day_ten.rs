use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayTen {
    pub day: Day,
}

fn next(
    m: &Vec<Vec<i64>>,
    prev: i64,
    i: usize,
    j: usize,
    bounds: (usize, usize),
    visited_destination: &mut HashSet<(usize, usize)>,
    calculate_all: bool,
) -> i64 {
    let actual = m[i][j];
    if visited_destination.contains(&(i, j)) && !calculate_all {
        return 0;
    }

    if actual - prev != 1 {
        return 0;
    }

    if actual == 9 {
        visited_destination.insert((i, j));
        return 1;
    }

    return if i + 1 < bounds.1 {
        next(
            m,
            m[i][j],
            i + 1,
            j,
            bounds,
            visited_destination,
            calculate_all,
        )
    } else {
        0
    } + if i > 0 {
        next(
            m,
            m[i][j],
            i - 1,
            j,
            bounds,
            visited_destination,
            calculate_all,
        )
    } else {
        0
    } + if j + 1 < bounds.0 {
        next(
            m,
            m[i][j],
            i,
            j + 1,
            bounds,
            visited_destination,
            calculate_all,
        )
    } else {
        0
    } + if j > 0 {
        next(
            m,
            m[i][j],
            i,
            j - 1,
            bounds,
            visited_destination,
            calculate_all,
        )
    } else {
        0
    };
}

impl DayTen {
    fn read_input(&self) -> Result<Vec<Vec<i64>>> {
        let mut outer: Vec<Vec<i64>> = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let mut inner = vec![];

            let values: Vec<char> = line.chars().collect();
            for i in values.iter() {
                inner.push(i.to_digit(10).expect("parse char") as i64);
            }
            outer.push(inner)
        }
        Ok(outer)
    }
}

impl DayTrait<DayTen> for DayTen {
    fn new() -> DayTen {
        DayTen {
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
        let h = input.len();
        let w = input[0].len();

        let mut starting_points: Vec<(usize, usize)> = vec![];
        // find all starting points
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == 0 {
                    starting_points.push((i, j));
                }
            }
        }

        let mut p1 = 0;
        let mut p2 = 0;
        for sp in starting_points {
            let mut visited_destination: HashSet<(usize, usize)> = HashSet::new();
            p1 += next(
                &input,
                -1,
                sp.0,
                sp.1,
                (w, h),
                &mut visited_destination,
                false,
            );
            p2 += next(
                &input,
                -1,
                sp.0,
                sp.1,
                (w, h),
                &mut visited_destination,
                true,
            );
        }

        (p1, p2)
    }
}
