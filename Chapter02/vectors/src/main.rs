fn main() {
    let mut my_vec: Vec<i32> = (0..10).collect();
    println!("{:?}", my_vec); 
    my_vec.push(13);
    my_vec.push(21);
    println!("{:?}", my_vec); 
    let mut twenty_one = my_vec.pop(); // removes the last value
    println!("twenty_one= {:?}", twenty_one); 
    println!("{:?}", my_vec); 
}
