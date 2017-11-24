trait StaticObject {
    fn static_method(&self) -> String;
}

impl StaticObject for u8 {
    fn static_method(&self) -> String {
        format!("u8 : {}, ", *self)
    }
}

impl StaticObject for String {
    fn static_method(&self) -> String {
        format!("string : {}", *self)
    }
}

fn display_code<T: StaticObject>(data: T) {
    println!("{}", data.static_method());
}

fn main() {
    let test_one = 8u8;
    let test_two = "Some text".to_string();

    display_code(test_one);
    display_code(test_two);
}
