use std::f64::consts::PI;

fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

fn main() {
    let radius = 5.0;
    let area = circle_area(radius);
    println!("Circle area with radius {radius}: {area}");
}
