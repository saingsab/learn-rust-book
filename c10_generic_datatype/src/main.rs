struct Point<T> {
    x: T,
    y: T,
}

struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 { 
            x: self.x, 
            y: other.y, 
        }
    }
}
impl<T> Point<T>  {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point{x: 5, y: 10};
    println!("The value of p.x: {:?}", p.x);

    let p1 = Point1 {x: 5, y: 10.4};
    let p2 = Point1 {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
