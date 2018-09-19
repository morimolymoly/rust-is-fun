fn main() {
    let max = largest(&vec![10, 20, 4, 100]);
     println!("最大値は{}です", max);
     assert_eq!(max, 100);

     let point = Point{x: 10, y: 20};
     println!("xは{}です\nyは{}です", point.x(), point.y());

     let point = Point{x: 20.0, y: 30.0};
     println!("距離は{}です", point.distance_from_origin());

     println!("でぇけのは{}", largest2(&vec![40, 80, 90]));
     print_iroiro(&10, &20);
}

fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn largest2<T>(list: &[T]) -> T
    where T : PartialOrd + Copy
{
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn print_iroiro<T1, T2>(arg1: &T1, arg2: &T2)
    where T1: std::fmt::Display,
          T2: std::fmt::Display
{
    println!("{}", arg1);
    println!("{}", arg2);
}

struct Point<T> {
    x: T,
    y: T
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


