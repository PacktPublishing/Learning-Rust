fn main() {
    let varname: &f32;
    {
        let x = 3.14f32;
        varname = &x;
    }
    println!("varname = {}", varname);
}
