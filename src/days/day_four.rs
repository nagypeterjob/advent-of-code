use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayFour {
    pub day: Day,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Matrix {
    data: Vec<Vec<char>>,
}

impl Matrix {
    fn interesct(&self, hay: &str, x: i64, y: i64, p: &Point) -> bool {
        for i in 0..hay.len() {
            let nx = x + (i as i64 * p.x);
            let ny = y + (i as i64 * p.y);
            let nth_char = hay.chars().nth(i).expect("nth character available");
            if nx < 0
                || ny < 0
                || nx >= self.h()
                || ny >= self.w()
                || self.data[nx as usize][ny as usize] != nth_char
            {
                return false;
            }
        }
        return true;
    }
    fn interesct2(&self, x: i64, y: i64) -> bool {
        let x = x as usize;
        let y = y as usize;
        // M . S
        // . A .
        // M . S
        if (self.data[x][y] == 'A'
            && self.data[x - 1][y - 1] == 'M'
            && self.data[x + 1][y - 1] == 'M'
            && self.data[x - 1][y + 1] == 'S'
            && self.data[x + 1][y + 1] == 'S')
            || (
                // [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']]
                // S . S
                // . A .
                // M . M
                self.data[x][y] == 'A'
                    && self.data[x - 1][y - 1] == 'S'
                    && self.data[x + 1][y - 1] == 'M'
                    && self.data[x - 1][y + 1] == 'S'
                    && self.data[x + 1][y + 1] == 'M'
            )
            || (
                // [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']]
                // S . M
                // . A .
                // S . M
                self.data[x][y] == 'A'
                    && self.data[x - 1][y - 1] == 'S'
                    && self.data[x + 1][y - 1] == 'S'
                    && self.data[x - 1][y + 1] == 'M'
                    && self.data[x + 1][y + 1] == 'M'
            )
            || (
                // [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']]
                // M . M
                // . A .
                // S . S
                self.data[x][y] == 'A'
                    && self.data[x - 1][y - 1] == 'M'
                    && self.data[x + 1][y - 1] == 'S'
                    && self.data[x - 1][y + 1] == 'M'
                    && self.data[x + 1][y + 1] == 'S'
            )
        {
            return true;
        }

        false
    }
    fn w(&self) -> i64 {
        self.data[0].len() as i64
    }
    fn h(&self) -> i64 {
        self.data.len() as i64
    }
}

fn directions() -> Vec<Point> {
    return vec![
        Point { x: 0, y: 1 },   // horizontal Left
        Point { x: 0, y: -1 },  // horizontal right
        Point { x: -1, y: 0 },  // vertical up
        Point { x: 1, y: 0 },   // vertical down
        Point { x: -1, y: -1 }, // diagonal upper left
        Point { x: -1, y: 1 },  // diagonal upper right
        Point { x: 1, y: -1 },  // diagonal bottom left
        Point { x: 1, y: 1 },   // diagonal bottom right
    ];
}

impl DayFour {
    fn read_input(&self) -> Result<Vec<Vec<char>>> {
        let mut outer: Vec<Vec<char>> = vec![];
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let inner: Vec<char> = line.chars().collect();
            outer.push(inner);
        }
        Ok(outer)
    }
}

impl DayTrait<DayFour> for DayFour {
    fn new() -> DayFour {
        DayFour {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        let m = Matrix {
            data: self.read_input().expect("building character matrix"),
        };

        let hay = "XMAS";
        let mut count: i64 = 0;
        for (i, row) in m.data.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                count += directions()
                    .into_iter()
                    .filter(|p| m.interesct(hay, i as i64, j as i64, p))
                    .count() as i64;
            }
        }

        count
    }
    fn part_two(&self) -> i64 {
        let m = Matrix {
            data: self.read_input().expect("building character matrix"),
        };

        let mut count: i64 = 0;
        for x in 1..m.h() - 1 {
            for y in 1..m.w() - 1 {
                if m.interesct2(x, y) {
                    count += 1;
                }
            }
        }

        count
    }
}
