fn main() {
    let mut mutvar = 5;
    {
        println!("{}", mutvar); // outputs 5
        let y = &mut mutvar; // creates the mutable ref to mutvar
        *y += 1; // adds one to the reference and passes it back in to mutvar
    }
    println!("{}", mutvar); // outputs 6
}
