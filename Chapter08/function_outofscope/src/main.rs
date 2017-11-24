fn transfer_vec(v: Vec<i32>) {
    println!("v[0] in transfer_vec = {}", v[0]);
}

fn main() {
    let myvec = vec![1i32, 2i32, 3i32];
    transfer_vec(myvec);
    println!("myvec[0] is: {}", myvec[0]);
}
