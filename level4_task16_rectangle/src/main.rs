struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let r1 = Rectangle { width: 10.0, height: 5.0 };
    let r2 = Rectangle { width: 3.0, height: 2.0 };
    println!("Area: {}", r1.area());
    println!("Perimeter: {}", r1.perimeter());
    println!("Can hold r2: {}", r1.can_hold(&r2));
}
