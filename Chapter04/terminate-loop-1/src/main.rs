fn main() {
    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9];
    let mut counter: usize = 0;
    let mut result = 0f32;
    let mut quit = false;

    while quit != true {
        if my_array[counter] > 1.5 {
            quit = true;
        }
        else {
            result += my_array[counter];
            counter += 1;
        }
    }
     println!("{}", result);
}
