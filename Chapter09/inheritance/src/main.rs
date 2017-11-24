trait One {
    fn one(&self);
}
trait OneTwo: One {
    fn onetwo(&self);
}

struct Three;
impl One for Three {
    fn one(&self) {
        println!("one");
    }
}
impl OneTwo for Three {
    fn onetwo(&self) {
        println!("onetwo");
    }
}

fn main() {
    let t1 = Three {};

    t1.one();
    t1.onetwo();
}
