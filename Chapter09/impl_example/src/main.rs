struct Pair {
    x: f64,
    y: u64
}

impl Pair {
    fn print(&self) {
        println!("x = {}, y = {}", self.x, self.y)
    }
}

fn main() {
    let pair = Pair { x: 3.1415, y: 3 };
    pair.print();

}
