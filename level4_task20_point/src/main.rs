struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::origin();
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p1.distance(&p2));
}
