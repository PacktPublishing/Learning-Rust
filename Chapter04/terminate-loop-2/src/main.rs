fn main() {
    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9];
    let mut result = 0f32;

    for(_, value) in my_array.iter().enumerate() {
       if *value > 1.5 {
           break;
       }
       else {
           result += *value;
       }
    }

    println!("{}", result);
}
