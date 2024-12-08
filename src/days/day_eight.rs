use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayEight {
    pub day: Day,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn diff(&self, p: &Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
    fn within_bounds(&self, w: i64, h: i64) -> bool {
        return self.x < h && self.x >= 0 && self.y < w && self.y >= 0;
    }
    fn add(&self, p: &Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
    fn sub(&self, p: &Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

fn incr(p: Point, d: Point, w: i64, h: i64, nodes: &mut IndexMap<Point, i8>) {
    let np = p.add(&d);
    if !np.within_bounds(w, h) {
        return;
    }
    nodes.insert(np.clone(), 0);
    incr(np, d, w, h, nodes)
}

fn decr(p: Point, d: Point, w: i64, h: i64, nodes: &mut IndexMap<Point, i8>) {
    let np = p.sub(&d);
    if !np.within_bounds(w, h) {
        return;
    }
    nodes.insert(np.clone(), 0);
    decr(np, d, w, h, nodes)
}

impl DayEight {
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

impl DayTrait<DayEight> for DayEight {
    fn new() -> DayEight {
        DayEight {
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
        let m = self.read_input().expect("read input");
        let mut pairs: IndexMap<(Point, Point), char> = IndexMap::new();
        let mut antinode: IndexMap<Point, i8> = IndexMap::new();
        let mut all_antinodes: IndexMap<Point, i8> = IndexMap::new();

        let height = m.len();
        let width = m[0].len();

        let search = |x: usize,
                      y: usize,
                      hay: char,
                      m: &Vec<Vec<char>>,
                      pairs: &mut IndexMap<(Point, Point), char>| {
            for (i, row) in m.iter().enumerate() {
                for (j, &ch) in row.iter().enumerate() {
                    if ch == hay && (i != x || j != y) {
                        let (p1, p2) = (
                            Point {
                                x: x as i64,
                                y: y as i64,
                            },
                            Point {
                                x: i as i64,
                                y: j as i64,
                            },
                        );

                        if !pairs.contains_key(&(p1.clone(), p2.clone()))
                            && !pairs.contains_key(&(p2.clone(), p1.clone()))
                        {
                            pairs.insert((p1, p2), hay);
                        }
                    }
                }
            }
        };

        use std::time::Instant;
        let now = Instant::now();

        for (i, row) in m.iter().enumerate() {
            for (j, &node_id) in row.iter().enumerate() {
                if node_id != '.' {
                    search(i, j, node_id, &m, &mut pairs);
                }
            }
        }

        for (p1, p2) in pairs.keys().map(|k| (k.0, k.1)) {
            let d = p1.diff(&p2);

            all_antinodes.insert(p1.clone(), 0);
            all_antinodes.insert(p2.clone(), 0);

            incr(
                p1.clone(),
                d.clone(),
                width as i64,
                height as i64,
                &mut all_antinodes,
            );
            decr(
                p2.clone(),
                d.clone(),
                width as i64,
                height as i64,
                &mut all_antinodes,
            );

            let (a1, a2) = if p1.x < p2.x {
                (p1.add(&d), p2.sub(&d))
            } else {
                (p2.add(&d), p1.sub(&d))
            };

            if a1.within_bounds(width as i64, height as i64) {
                antinode.insert(a1, 0);
            }
            if a2.within_bounds(width as i64, height as i64) {
                antinode.insert(a2, 0);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        (
            antinode.iter().count() as i64,
            all_antinodes.iter().count() as i64,
        )
    }
}
