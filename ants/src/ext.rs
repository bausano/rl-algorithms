use rand::prelude::*;

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
