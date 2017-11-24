fn do_something(number: f32) -> f32 {
    number + 32f32
}

fn main() {
    let num = 10f32;
    let numtwo = do_something(num);
    println!("num is: {}", num);
    println!("numtwo is : {}", numtwo);
}
