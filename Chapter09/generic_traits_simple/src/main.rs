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


fn main() {
    let peri = Shape {
        line_one: 5,
        line_two: 12,
    };
    println!("Side 1 = 5, Side 2 = 12, Perimeter = {}", peri.calc());

}
