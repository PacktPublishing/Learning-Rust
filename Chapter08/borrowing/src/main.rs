fn sumprod(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0i32, |a, &b| a + b);
    let product = v2.iter().fold(1i32, |a, &b| a * b);
    return sum + product;
}

fn main() {
    let vecone = vec![2, 3, 5];
    let vectwo = vec![3, 5];
    let ans = sumprod(&vecone, &vectwo);
    println!("ans = {}", ans);
}
