use std::ops::Neg;

#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Copy + Neg<Output = T>> Point<T> {
    fn clockwise(&self) -> Point<T> {
        Point {
            x: self.y.neg(),
            y: self.x,
        }
    }

    fn counterclockwise(&self) -> Point<T> {
        Point {
            x: self.y,
            y: self.x.neg(),
        }
    }
}

fn main() {
    // Test for i32 points
    let point_i32 = Point { x: 3, y: 4 };
    let clockwise_point_i32 = point_i32.clockwise();
    let counterclockwise_point_i32 = point_i32.counterclockwise();

    println!("Original i32 Point: {:?}", point_i32);
    println!("Clockwise i32 Point: {:?}", clockwise_point_i32);
    println!("Counterclockwise i32 Point: {:?}", counterclockwise_point_i32);

    assert_eq!(clockwise_point_i32.x, -4);
    assert_eq!(clockwise_point_i32.y, 3);

    assert_eq!(counterclockwise_point_i32.x, 4);
    assert_eq!(counterclockwise_point_i32.y, -3);

    // Test for f64 points
    let point_f64 = Point::<f64> { x: 3.5, y: 2.0 };
    let clockwise_point_f64 = point_f64.clockwise();
    let counterclockwise_point_f64 = point_f64.counterclockwise();

    println!("Original f64 Point: {:?}", point_f64);
    println!("Clockwise f64 Point: {:?}", clockwise_point_f64);
    println!("Counterclockwise f64 Point: {:?}", counterclockwise_point_f64);

    assert!((clockwise_point_f64.x + 2.0).abs() < f64::EPSILON);
    assert!((clockwise_point_f64.y - 3.5).abs() < f64::EPSILON);

    assert!((counterclockwise_point_f64.x - 2.0).abs() < f64::EPSILON);
    assert!((counterclockwise_point_f64.y + 3.5).abs() < f64::EPSILON);
}
