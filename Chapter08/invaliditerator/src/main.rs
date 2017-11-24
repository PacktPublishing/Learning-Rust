fn main() {
    let mut myvec = vec![5i32, 10i32, 15i32, 20i32, 25i32, 30i32];

    for i in &myvec {
        println!("i = {}", i);
        myvec.push(35i32);
    }
}
