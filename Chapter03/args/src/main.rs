use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("There was {:?} arguments passed in. They were {:?}.", args.len() - 1, &args[1..]);
}
