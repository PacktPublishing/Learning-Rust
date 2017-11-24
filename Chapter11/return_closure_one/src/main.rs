fn add_five_closure() -> (Fn(i32) -> i32) {
    let num = 5;
    |x| x + num
}
fn main() {
    let test = add_five_closure();
    let f = test(5);
    println!("{}", f);
}
