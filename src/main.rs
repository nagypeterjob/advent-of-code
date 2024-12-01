mod days;

use days::{day::DayTrait, day_one::DayOne};

fn main() {
    let day_one_instance = DayOne::new();
    println!("{}", day_one_instance.display());
    let (p1, p2) = (day_one_instance.part_one(), day_one_instance.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);
}
