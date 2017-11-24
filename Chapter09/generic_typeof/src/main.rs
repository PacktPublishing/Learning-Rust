use std::any::TypeId;

fn main() {
    println!("{:?}", TypeId::of::<i32>());
    println!("{:?}", TypeId::of::<str>());
    println!("{:?}", TypeId::of::<String>());
}
