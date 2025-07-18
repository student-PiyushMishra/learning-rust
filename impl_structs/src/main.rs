struct Rect {
    length: i32,
    width: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.length * self.width
    }
}

fn main() {
    let rectangle: Rect = Rect {
        length: 50,
        width: 100,
    };
    println!("Area of rectangle: {}", rectangle.area());
}
