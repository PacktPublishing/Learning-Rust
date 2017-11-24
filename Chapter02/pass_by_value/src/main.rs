fn main()
{
    let add = add_values(3, 5); 
    println!("{:?}", add);
}

fn add_values(a: i32, b: i32) -> i32
{
    a + b
}
