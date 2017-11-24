struct Shape<T> {
    line_one: T,
    line_two: T,
}

trait Calculate<T> {
    fn calc(&self) -> T;
}

impl Calculate<i32> for Shape<i32> {
    fn calc(&self) -> i32 {
        self.line_one * 2 + self.line_two * 2
    }
}

impl Calculate<f32> for Shape<f32> {
fn calc(&self) -> f32 {
    3.1415927f32 * self.line_one * self.line_two
}
}


fn main() {
    let peri_int = Shape {
        line_one: 5,
        line_two: 12,
    };
    let peri_float = Shape {
        line_one: 5.1f32,
        line_two: 12.3f32,
    };
    println!("line_one 1 = 5, Side 2 = 12, Perimeter = {}", peri_int.calc());
    println!("line_one 1 = 5.1, Side 2 = 12.3, Perimeter = {}", peri_float.calc());

}
