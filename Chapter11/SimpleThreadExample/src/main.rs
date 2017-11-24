use std::thread;
fn main() {
    thread::spawn(|| {
        println!("Hello from a thread in your Rust program");
    });
}
