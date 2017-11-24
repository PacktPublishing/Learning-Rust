fn do_something(number: i32) -> i32 {
    number + 32
}

fn main() {
    let num = 10i32;
    let numtwo = do_something(num);
    println!("num is: {}", num);
    println!("numtwo is : {}", numtwo);
}
