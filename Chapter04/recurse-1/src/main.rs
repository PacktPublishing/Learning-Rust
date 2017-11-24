fn recurse(n: i32) {
     let v = match n % 2 
     {
         0 => n / 2,
         _ => 3 * n + 1
     };
     println!("{}", v);
  
     if v != 1 { 
         recurse(v) 
     }
}

fn main() { 
     recurse(25) 
}
