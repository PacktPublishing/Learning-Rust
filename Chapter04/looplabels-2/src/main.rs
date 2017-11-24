fn main()  {
    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9, 1.3, 0.1, 1.6, 0.6, 0.9, 1.1, 1.31, 1.49, 1.5, 0.7];
    let my_time = vec![0.2f32, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 3.0, 3.2, 3.4, 3.6, 3.8];

    'time_loop: for(_, time_value) in my_time.iter().enumerate() {
       'data_loop: for(_, value) in my_array.iter().enumerate() {
           if *value < 1.5 {
               continue 'data_loop;
           }
           if *time_value % 5f32 == 0f32 {
               continue 'time_loop;
           }
           println!("Data point = {} at time {}s", *value, *time_value);
       }
    }
}
