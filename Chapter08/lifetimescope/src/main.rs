struct MyStruct<'a> {
    lifea: &'a i32,
}

fn main() {
    let x;
    {
        let y = &5; // means let y = 5; let y = &y;
        let f = MyStruct { lifea: y };
        x = &f.lifea
    }
    println!("{}", x);
}
