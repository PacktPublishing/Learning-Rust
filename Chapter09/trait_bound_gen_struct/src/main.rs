#[derive(PartialEq)]
struct Four {
    x: u32,
    y: i32,
    z: f32,
    g: f64,
}


fn main() {
    let test_one = Four {
        x: 5,
        y: 6,
        z: 1.4,
        g: 592.2,
    };
    let test_two = Four {
        x: 5,
        y: 6,
        z: 0.0,
        g: 592.2,
    };

    if test_one == test_two {
        println!("test_one == test_two");
    } else {
        println!("test_one != test_two");
    }

    let test_two = Four {
        x: 5,
        y: 6,
        z: 1.4,
        g: 592.2,
    };
    if test_one == test_two {
        println!("test_one == test_two");
    } else {
        println!("test_one != test_two");
    }
}
