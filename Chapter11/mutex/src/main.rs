use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let primes = Arc::new(Mutex::new(vec![1, 2, 3, 5, 7, 9, 13, 17, 19, 23]));

    for i in 0..10 {
        let primes = primes.clone();
        thread::spawn(move || {
            let mut data = primes.lock().unwrap();
            data[0] += i;
        });
    }
    thread::sleep(Duration::from_millis(50));
}
