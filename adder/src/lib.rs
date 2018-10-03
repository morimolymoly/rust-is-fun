#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    #[test]
    fn test_area() {
        let r1 = Rectangle{length:10, width:20};
        assert_eq!(r1.calc_area(), 200);
    }
}

pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn calc_area(self) -> u32 {
        self.length * self.width
    }
}
