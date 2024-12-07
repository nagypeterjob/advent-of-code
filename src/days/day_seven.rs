use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};

use std::fs::File;
use std::io::{self, BufRead};

pub struct DaySeven {
    pub day: Day,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Multiplication,
    Addition,
    Concatenation,
}

impl DaySeven {
    fn read_input(&self) -> Result<Vec<(i64, Vec<i64>)>> {
        let mut outer: Vec<(i64, Vec<i64>)> = vec![];

        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let values: Vec<&str> = line.split(":").collect();
            let sum = values[0].parse::<i64>()?;
            let numbers: Vec<i64> = values[1]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i64>().expect("parse test values"))
                .collect();
            outer.push((sum, numbers));
        }
        Ok(outer)
    }
}

fn true_operation(target: i64, n: &Vec<i64>, operations: &mut [Operation], depth: usize) -> bool {
    if depth == operations.len() {
        let mut result = n[0];
        for i in 0..operations.len() {
            match operations[i] {
                Operation::Addition => result += n[i + 1],
                Operation::Multiplication => result *= n[i + 1],
                Operation::Concatenation => (),
            }
        }

        return result == target;
    }

    if [Operation::Addition, Operation::Multiplication]
        .iter()
        .any(|op| {
            operations[depth] = *op;
            true_operation(target, n, operations, depth + 1)
        })
    {
        return true;
    }

    false
}

fn true_operation2(target: i64, n: &Vec<i64>, operations: &mut [Operation], depth: usize) -> bool {
    let con = |a: i64, b: i64| -> i64 { a * 10_i64.pow(b.ilog10() + 1) + b };

    if depth == operations.len() {
        let mut result = n[0];
        for i in 0..operations.len() {
            match operations[i] {
                Operation::Addition => result += n[i + 1],
                Operation::Multiplication => result *= n[i + 1],
                Operation::Concatenation => result = con(result, n[i + 1]),
            }
        }
        return result == target;
    }

    if [
        Operation::Addition,
        Operation::Multiplication,
        Operation::Concatenation,
    ]
    .iter()
    .any(|op| {
        operations[depth] = *op;
        true_operation2(target, n, operations, depth + 1)
    }) {
        return true;
    }

    false
}

impl DayTrait<DaySeven> for DaySeven {
    fn new() -> DaySeven {
        DaySeven {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        let m = self.read_input().expect("read input");
        use std::time::Instant;
        let now = Instant::now();

        let sum = m
            .iter()
            .filter(|(s, n)| true_operation(*s, n, &mut vec![Operation::Addition; n.len() - 1], 0))
            .map(|(s, _)| s)
            .sum::<i64>();

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        sum
    }
    fn part_two(&self) -> i64 {
        let m = self.read_input().expect("read input");
        use std::time::Instant;
        let now = Instant::now();

        let sum = m
            .iter()
            .filter(|(s, n)| true_operation2(*s, n, &mut vec![Operation::Addition; n.len() - 1], 0))
            .map(|(s, _)| s)
            .sum::<i64>();

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        sum
    }
}
