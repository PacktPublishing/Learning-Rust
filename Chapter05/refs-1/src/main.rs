fn main() {
    let x = 2;
    let y: &i32; {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        y = &(x_cube + x_squared + x); // this value goes away after this line
    };
    println!("Y = {}", *y);
}
