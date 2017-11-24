fn sumprod(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    let sum = v1.iter().fold(0i32, |a, &b| a + b);
    let product = v2.iter().fold(1i32, |a, &b| a * b);
    return (v1, v2, sum + product); // return ownership
}

fn main() {
    let vecone = vec![2, 3, 5];
    let vectwo = vec![3, 5];
    let (vecone, vectwo, ans) = sumprod(vecone, vectwo); // pass ownership
    println!("ans = {}", ans);
}
