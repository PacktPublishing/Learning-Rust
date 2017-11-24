extern crate num;
use std::ops::{Add, Mul};
use num::FromPrimitive;

struct Shape<T> {
    line_one: T,
    line_two: T,
}

trait Calculate<T> {
    fn calc(&self) -> T;
}

impl<T> Calculate<T> for Shape<T>
where
    T: Copy + FromPrimitive + Add<Output = T> + Mul<Output = T>,
{
    fn calc(&self) -> T {
        let two = T::from_u8(2).expect("Unable to create a value of 2");
        self.line_one * two + self.line_two * two
    }
}


fn main() {
    let peri = Shape {
        line_one: 5,
        line_two: 12,
    };
    println!("line_one = 5, line_two = 12, perimeter = {}", peri.calc());
}
