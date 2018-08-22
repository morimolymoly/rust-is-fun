#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    score: u32,
    have_pc: bool
}
impl Student {
    // selfを借用
    fn say_hello(&self) {
        println!("hello! my name is {}", self.name);
    }
    fn say_hello_with_move(self) {
        println!("hello! my name is {}", self.name);
    }
    fn throw_pc_from_window(&mut self) {
        self.have_pc = false;
    }
}

struct Point(u32, u32, u32);

fn main() {
    let mut taro = Student {
        name: String::from("太郎"),
        age: 12,
        score: 100,
        have_pc: true
    };
    taro.say_hello();
    taro.throw_pc_from_window();
    // taroがmoveされるのでだめ
    //taro.say_hello_with_move();
    println!("taro is {:#?}", taro);
    let taro_kai = Student {
        score: 40,
        ..taro
    };
    println!("taro kai's score is {}", taro_kai.score);

    let point1 = Point(10, 20, 30);
    print_point(&point1);
}

fn print_point(p: &Point) {
    println!("Point x:{}, y:{}, z{}", p.0, p.1, p.2);
}
