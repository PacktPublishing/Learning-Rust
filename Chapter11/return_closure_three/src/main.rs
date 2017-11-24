fn add_five_closure() -> Box<(Fn(i32) -> i32)> {
    let num = 5;
    Box::new(move |x| x + num)
}
fn main() {
    let test = add_five_closure();
    let f = test(5);
    println!("{}", f);
}
