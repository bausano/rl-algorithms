use rand::prelude::*;
use std::cmp::Ordering;
use std::ops::Sub;

pub trait ThreadRngExt {
    /// Input must be a number p where `1 >= p >= 0`. It generates a range from
    /// 0 to 1. If the range is less than given probability, returns true,
    /// otherwise false.
    fn roll_dice(&mut self, probability: f32) -> bool;
}

impl ThreadRngExt for ThreadRng {
    fn roll_dice(&mut self, probability: f32) -> bool {
        probability > self.gen_range(0.0, 1.0)
    }
}

pub trait NumExt
where
    Self: Sub<Output = Self> + PartialOrd<Self> + Copy + Sized,
{
    fn partial_ord(self, other: Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl NumExt for usize {}
