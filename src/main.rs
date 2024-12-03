mod days;

use days::{day::DayTrait, day_one::DayOne, day_three::DayThree, day_two::DayTwo};

fn main() {
    let day = DayOne::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayTwo::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayThree::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);
}
