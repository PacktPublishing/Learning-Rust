use std::thread;
fn main() {
    let threadhandle = thread::spawn(|| "Hello from a thread in your Rust program");

    println!("{}", threadhandle.join().unwrap());
}
