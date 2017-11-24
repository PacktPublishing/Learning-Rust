struct MyPi(f32);
fn main() {
    let my_pi = MyPi(22f32 / 7f32);
    let MyPi(pi) = my_pi;
    println!("pi = {}", pi);
}
