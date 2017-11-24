use std::thread;
use std::rc::Rc;

struct MyCounter
{
    count: i32
}

fn wont_work()
{
    let mut counter = Rc::new(MyCounter {count: 0});
    thread::spawn(move || // new thread -- fails to compile here
    {
        counter.count += 1;
    });
    println!("{}", counter.count);
}
