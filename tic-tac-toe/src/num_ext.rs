use std::cmp::Ordering;
use std::ops::Sub;

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

impl NumExt for f32 {}
