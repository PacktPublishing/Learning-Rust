fn main() {
    let mut my_mut_num = 10;
    {
        let mut sub_num = move |x: i32| my_mut_num -= x;
        sub_num(3);
    }
    println!("{}", my_mut_num);
}
