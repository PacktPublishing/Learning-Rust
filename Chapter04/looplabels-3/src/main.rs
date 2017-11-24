fn main()  {
    let my_array = vec![0.6f32, 0.4, 0.2, 0.8, 1.3, 1.1, 1.7, 1.9, 1.3, 0.1, 1.6, 0.6, 0.9, 1.1, 1.31, 1.49, 1.5, 0.7];
    let my_time = vec![0.2f32, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 3.0, 3.2, 3.4, 3.6, 3.8];
    let mut my_new_array = vec![];
    let mut my_new_time = vec![];

    'time_loop: for(t, _) in my_time.iter().enumerate() {
       'data_loop: for(v, value) in my_array.iter().enumerate() {
           if *value < 1.5 {
               continue 'data_loop;
           }
           else {
               if t % 5 != 0 {
                    my_new_array.push(*value);
                    my_new_time.push(my_time[v]);
               }       
           }
       
           if v == my_array.len() {
               break;
           }
       }
    }
    
    for(m, my_data) in my_new_array.iter().enumerate() {
        println!("Data = {} at time {}", *my_data, my_new_time[m]);
    }
}
