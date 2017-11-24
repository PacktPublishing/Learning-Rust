fn main() {
    'outer_loop: for x in 0..10 {
        'inner_loop: for y in 0..10 {
            if x % 2 == 0 { continue 'outer_loop; } 
            if y % 2 == 0 { continue 'inner_loop; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
