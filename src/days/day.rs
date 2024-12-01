use std::fmt;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Debug)]
pub struct Day {
    pub id: u64,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub trait DayTrait<T> {
    fn new() -> T;
    fn display(&self) -> String;
    fn part_one(&self) -> i64;
    fn part_two(&self) -> i64;
}

pub fn increment_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, SeqCst)
}
