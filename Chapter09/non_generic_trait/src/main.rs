struct Perimeter {
    side_one: i32,
    side_two: i32,
}
struct Oval {
    radius: f32,
    height: f32,
}

trait CalcPerimeter {
    fn calc_perimeter(&self) -> i32;
}
trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcPerimeter for Perimeter {
    fn calc_perimeter(&self) -> i32 {
        self.side_one * 2 + self.side_two * 2
    }
}

impl CalcArea for Oval {
    fn calc_area(&self) -> f32 {
        3.1415927f32 * self.radius * self.height
    }
}

fn main() {
    let peri = Perimeter {
        side_one: 5,
        side_two: 12,
    };
    println!(
        "Side 1 = 5, Side 2 = 12, Perimeter = {}",
        peri.calc_perimeter()
    );
    let area = Oval {
        radius: 5.1f32,
        height: 12.3f32,
    };
    println!("Radius = 5.1, Height = 12.3, Area = {}", area.calc_area());
}
