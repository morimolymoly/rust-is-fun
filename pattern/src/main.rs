fn main() {
    let red = Color::Red;
    println!("red value is {}", red.get_color_value());

    let mori_phone = SmartPhone::android(
        Android{
            owners_name: String::from("mori"),
            price: 30000,
            device_name: String::from("HTC 10")
        }
    );

    let three: Option<u32> = Some(3);
    let four = inc(three);
    let five = inc(four);
    let none = inc(None);
    if let Some(5) = five {
        println!("FIVE!!");
    }
}

enum Color{
    Red,
    Green,
    Yellow,
}
impl Color {
    fn get_color_value(&self) -> i32{
        match self {
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
        }
    }
}

struct iPhone {
    owners_name: String,
    price: i32
}

struct Android {
    owners_name: String,
    price: i32,
    device_name: String
}

enum SmartPhone {
    iphone(iPhone),
    android(Android),
}

fn inc(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
