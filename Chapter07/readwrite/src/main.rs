mod readwrite;
use readwrite::*;

fn main() {
    store_y(6);
    println!("Y is now {}", new_y());
}
