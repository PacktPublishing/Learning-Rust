fn call_with_three<F>(some_closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    some_closure(3)
}
fn main() {
    let answer = call_with_three(|x| x + 10);
    println!("{}", answer);
}
