use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DaySix {
    pub day: Day,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(mut self) -> Direction {
        self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self
    }
}

impl DaySix {
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

impl DayTrait<DaySix> for DaySix {
    fn new() -> DaySix {
        DaySix {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        /*let m = vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ];*/
        let m = self.read_input().expect("read matrix");
        let mut seen: IndexMap<(usize, usize), char> = IndexMap::new();
        let mut x: usize = 0;
        let mut y: usize = 0;

        // find starting position
        if let Some((i, row)) = m.iter().enumerate().find(|(_, row)| row.contains(&'^')) {
            if let Some(j) = row.iter().position(|&ch| ch == '^') {
                x = i;
                y = j;
            }
        }

        let height = m.len();
        let width = m.get(0).expect("matrix to have lines").len();

        let mut direction = Direction::Up;

        let move_position = |x: usize, y: usize, direction: &Direction| match direction {
            Direction::Up => (x.wrapping_sub(1), y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y.wrapping_sub(1)),
        };

        loop {
            let (nx, ny) = move_position(x, y, &direction);

            if nx >= width || ny >= height {
                break;
            }

            if m[nx][ny] == '#' {
                direction = direction.turn();
            } else {
                x = nx;
                y = ny;
                seen.insert((x, y), '.');
            }
        }
        seen.iter().count() as i64
    }
    fn part_two(&self) -> i64 {
        let m = self.read_input().expect("read matrix");

        let mut seen: IndexMap<(usize, usize), char> = IndexMap::new();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut initial_x = 0;
        let mut initial_y = 0;

        // find starting position
        if let Some((i, row)) = m.iter().enumerate().find(|(_, row)| row.contains(&'^')) {
            if let Some(j) = row.iter().position(|&ch| ch == '^') {
                x = i;
                y = j;
                initial_x = i;
                initial_y = j;
            }
        }

        let height = m.len();
        let width = m.get(0).expect("matrix to have lines").len();

        let move_position = |x: usize, y: usize, direction: &Direction| match direction {
            Direction::Up => (x.wrapping_sub(1), y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y.wrapping_sub(1)),
        };

        // First walk. Collects coordinates the guards steps on.
        let mut direction = Direction::Up;
        loop {
            let (nx, ny) = move_position(x, y, &direction);

            if nx >= width || ny >= height {
                break;
            }

            if m[nx][ny] == '#' {
                direction = direction.turn();
            } else {
                x = nx;
                y = ny;

                seen.insert((x, y), '.');
            }
        }

        let was_loop = |mut x: usize,
                        mut y: usize,
                        m: Vec<Vec<char>>,
                        mut direction: Direction,
                        mut turn_history: IndexMap<(usize, usize), Direction>|
         -> bool {
            loop {
                let (nx, ny) = move_position(x, y, &direction);

                if nx >= width || ny >= height {
                    // could exit nornally
                    return false;
                }

                if m[nx][ny] == '#' {
                    if turn_history.get(&(x, y)) == Some(&direction) {
                        // we have already seen this, loop
                        return true;
                    }
                    turn_history.insert((x, y), direction.clone());
                    direction = direction.turn();
                } else {
                    x = nx;
                    y = ny;
                }
            }
        };

        seen.iter()
            .filter(|&((seen_x, seen_y), _)| {
                let mut new_m = m.clone();
                new_m[*seen_x][*seen_y] = '#';
                was_loop(initial_x, initial_y, new_m, Direction::Up, IndexMap::new())
            })
            .count() as i64
        0
    }
}
