mod days;

use days::{
    day::DayTrait, day_eight::DayEight, day_five::DayFive, day_four::DayFour, day_nine::DayNine,
    day_one::DayOne, day_seven::DaySeven, day_six::DaySix, day_ten::DayTen, day_three::DayThree,
    day_two::DayTwo,
};

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

    let day = DayFour::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayFive::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DaySix::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DaySeven::new();
    println!("{}", day.display());
    let (p1, p2) = (day.part_one(), day.part_two());
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayEight::new();
    println!("{}", day.display());
    let (p1, p2) = day.solution();
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayNine::new();
    println!("{}", day.display());
    let (p1, p2) = day.solution();
    println!("Solution Part one: {}, Part two: {}", p1, p2);

    let day = DayTen::new();
    println!("{}", day.display());
    let (p1, p2) = day.solution();
    println!("Solution Part one: {}, Part two: {}", p1, p2);
}
