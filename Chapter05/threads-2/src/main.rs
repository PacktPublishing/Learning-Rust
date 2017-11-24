use std::thread;
use std::sync::{Arc, Mutex};

struct MyCounter
{
    count: i32
}

fn main()
{
    let counter = Arc::new(Mutex::new(MyCounter {count: 0}));
    let another_counter = counter.clone();
    thread::spawn(move || // new thread 
    {
        let mut counter = another_counter.lock().expect("Locking of cloned counter failed");
        counter.count += 1;
    });
    println!("{}", counter.lock().unwrap().count);
}
