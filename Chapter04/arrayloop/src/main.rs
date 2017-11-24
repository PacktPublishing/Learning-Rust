fn main() {
    let my_array = vec![1i32,3,5,7,9,11,13];
    let mut value = 0i32;

    for(_,line) in my_array.iter().enumerate()
    {
       value += *line;
    }
    println!("{}", value);
}
