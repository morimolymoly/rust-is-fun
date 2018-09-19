fn main() {
    println!("Hello, world!");
}

struct SugoiPoint<T, U> {
    x: T,
    y: U
}

impl <T, U> SugoiPoint<T, U> {
    fn new(x: &T, y: &U) -> SugoiPoint<T, U> {
        SugoiPoint{x: T::default(), y: U::default()}
    }
    fn mixup<V, W>(&self, other: &SugoiPoint<V, W>) -> SugoiPoint<T, W> {
        SugoiPoint::new(&self.x, &other.y)
    }
}
