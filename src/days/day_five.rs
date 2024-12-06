use crate::days::day::{increment_id, Day, DayTrait};
use anyhow::{Ok, Result};
use indexmap::IndexMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct DayFive {
    pub day: Day,
}

impl DayFive {
    fn read_input(&self) -> Result<(IndexMap<i64, Vec<i64>>, Vec<Vec<i64>>)> {
        let mut rules: IndexMap<i64, Vec<i64>> = IndexMap::new();
        let mut updates: Vec<Vec<i64>> = vec![];

        let mut rule_parser = true;
        let file = File::open(format!("src/input/{}.txt", self.day.id))?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            if line.is_empty() {
                rule_parser = false;
                continue;
            }

            if rule_parser {
                let values: Vec<&str> = line.split("|").collect();
                let a = values[0].parse::<i64>()?;
                let b = values[1].parse::<i64>()?;
                if rules.contains_key(&a) {
                    rules.get_mut(&a).unwrap().push(b);
                } else {
                    rules.insert(a, vec![b]);
                }
            } else {
                let update = line
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<i64>().expect("parse digit"))
                    .collect::<Vec<i64>>();
                updates.push(update);
            }
        }
        Ok((rules, updates))
    }
}

impl DayTrait<DayFive> for DayFive {
    fn new() -> DayFive {
        DayFive {
            day: Day { id: increment_id() },
        }
    }
    fn display(&self) -> String {
        return format!("Day #{}", self.day);
    }
    fn part_one(&self) -> i64 {
        let (rules, updates) = self.read_input().expect("read input");

        // lets use the fact, that if an x|y pair doesn't exist among the rule set,
        // then the update is invalid
        updates
            .iter()
            .filter_map(|update| {
                let is_valid = update
                    .windows(2)
                    .all(|pair| rules.get(&pair[0]).map_or(false, |r| r.contains(&pair[1])));

                if is_valid {
                    Some(update[update.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }
    fn part_two(&self) -> i64 {
        let (rules, updates) = self.read_input().expect("read input");

        // collect just the invalid updates
        let mut invalid: Vec<Vec<i64>> = updates
            .iter()
            .filter(|update| {
                update
                    .windows(2)
                    .any(|pair| !rules.get(&pair[0]).map_or(false, |r| r.contains(&pair[1])))
            })
            .cloned()
            .collect();

        // Use a few tricks here:
        // - the solution uses an indexmap which preserves insert order.
        // - given a ruleset (X|Y), for each X we build a map[X]Vec<Y |...> structure.
        // - given a valid update [x1, x2, x3 ..., xn], we know that:
        //   - vector for X1 will have to conatin [x2, x3,..., xn].
        //   - vector for X2 will have to contain [x3, x4,..., xn], etc.
        // - By sorting (desc) a given update vector's elements by the number of pages they preceede gives us the correct order.
        // - Recursion is a much more elegant solution, I just vanted to validate if this line of thought works.
        invalid
            .iter_mut()
            .map(|inv| {
                let cl = inv.clone();
                inv.sort_by(|a, b| {
                    let score = |item: &i64| {
                        rules.get_full(item).map_or(0, |x| {
                            x.2.iter().filter(|&&item| cl.contains(&item)).count()
                        })
                    };
                    score(b).cmp(&score(a))
                });
                inv[inv.len() / 2]
            })
            .sum()
    }
}
