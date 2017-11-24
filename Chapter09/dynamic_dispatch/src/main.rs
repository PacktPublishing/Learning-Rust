trait DynamicObject {
    fn dynamic_method(&self) -> String;
}

impl DynamicObject for u8 {
    fn dynamic_method(&self) -> String {
        format!("u8 : {}, ", *self)
    }
}

impl DynamicObject for String {
    fn dynamic_method(&self) -> String {
        format!("string : {}", *self)
    }
}

fn display_code(data: &DynamicObject) {
    println!("{}", data.dynamic_method());
}

fn main() {
    let test_one = 8u8;
    let test_two = "Some text".to_string();

    display_code(&test_one as &DynamicObject);
    display_code(&test_two as &DynamicObject);
}
