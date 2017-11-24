use std::ops::Mul;

fn multiply_generic<T: Mul<Output = T>>(a: T, b: T) -> T {
    return a * b;
}

fn main() {
   multiply_generic("123", "234");

}
