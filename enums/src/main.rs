use core::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Rectangle(f64, f64),
    Square(f64),
    Circle(f64),
}

fn main() {
    let rect: Shape = Shape::Rectangle(5.0, 2.0);
    let square: Shape = Shape::Square(10.0);
    let circle: Shape = Shape::Circle(7.5);
    let area_of_rect = calculate_area(rect);
    let area_of_square = calculate_area(square);
    let area_of_circle = calculate_area(circle);
    println!("{}", area_of_rect);
    println!("{}", area_of_square);
    println!("{}", area_of_circle);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(len, wid) => len * wid,
        Shape::Square(side_len) => side_len * side_len,
        Shape::Circle(radius) => PI * radius * radius,
    }
}
