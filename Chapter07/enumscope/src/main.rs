enum MyFirstEnum {
    TupleType(f32, i8, String),
    StuctType { varone: i32, vartwo: f64 },
    NewTypeTuple(i32),
    SomeVarName,
}

enum MySecondEnum {
    TupleType(f32, i8, String),
    StuctType { varone: i32, vartwo: f64 },
    NewTypeTuple(i32),
}

fn main() {
    let mut text1 = "".to_owned(); // text1: String
    let mut text2 = "".to_owned(); // text2: String
    let mut num1 = 0f32;

    let value = MyFirstEnum::TupleType(3.14, 1, "Hello".to_owned());
    let value2 = MySecondEnum::TupleType(6.28, 0, "World".to_owned());

    if let MyFirstEnum::TupleType(f, i, s) = value {
        text1 = s;
        num1 = f;
    }

    if let MySecondEnum::TupleType(f, i, s) = value2 {
        text2 = s;
    }

    println!("{} {} from the {} man", text1, text2, num1)
}
