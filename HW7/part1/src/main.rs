use std::f64::consts::PI;

#[derive(Debug, Clone)]
enum Shape {
    Triangle(f64, f64, f64),
    Rectangle(f64, f64),
    Circle(f64),
}

impl Shape {
    fn create_shape(shape_type: &str, params: Vec<f64>) -> Option<Shape> {
        match shape_type {
            "triangle" if params.len() == 3 => Some(Shape::Triangle(params[0], params[1], params[2])),
            "rectangle" if params.len() == 2 => Some(Shape::Rectangle(params[0], params[1])),
            "circle" if params.len() == 1 => Some(Shape::Circle(params[0])),
            _ => None,
        }
    }

    fn area(&self) -> f64 {
        match *self {
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            },
            Shape::Rectangle(width, height) => width * height,
            Shape::Circle(radius) => PI * radius * radius,
        }
    }

    fn perimeter(&self) -> f64 {
        match *self {
            Shape::Triangle(a, b, c) => a + b + c,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Circle(radius) => 2.0 * PI * radius,
        }
    }

    fn double_perimeter(&mut self) {
        match self {
            Shape::Triangle(a, b, c) => {
                *a *= 2.0;
                *b *= 2.0;
                *c *= 2.0;
            },
            Shape::Rectangle(width, height) => {
                *width *= 2.0;
                *height *= 2.0;
            },
            Shape::Circle(radius) => {
                *radius *= 2.0;
            },
        }
    }

    fn verify_parameters(&self) -> bool {
        match *self {
            Shape::Triangle(a, b, c) => a > 0.0 && b > 0.0 && c > 0.0 && (a + b) > c && (b + c) > a && (c + a) > b,
            Shape::Rectangle(width, height) => width > 0.0 && height > 0.0,
            Shape::Circle(radius) => radius > 0.0,
        }
    }
}

fn main() {
    let triangle = Shape::create_shape("triangle", vec![3.0, 4.0, 5.0]).unwrap();
    match triangle {
        Shape::Triangle(a, b, c) => println!("Triangle sides: {}, {}, {}", a, b, c),
        _ => (),
    }
    println!("Area of triangle: {}", triangle.area());
    println!("Perimeter of triangle: {}", triangle.perimeter());
    let mut triangle_copy = triangle.clone();
    triangle_copy.double_perimeter();
    println!("Doubled perimeter of triangle: {}", triangle_copy.perimeter());
    println!("Area with doubled perimeter: {}", triangle_copy.area());
    println!("Parameters of triangle are valid: {}", triangle.verify_parameters());

    let rectangle = Shape::create_shape("rectangle", vec![3.0, 4.0]).unwrap();
    match rectangle {
        Shape::Rectangle(width, height) => println!("Rectangle sides: {}x{}", width, height),
        _ => (),
    }
    println!("Area of rectangle: {}", rectangle.area());
    println!("Perimeter of rectangle: {}", rectangle.perimeter());
    let mut rectangle_copy = rectangle.clone();
    rectangle_copy.double_perimeter();
    println!("Doubled perimeter of rectangle: {}", rectangle_copy.perimeter());
    println!("Area with doubled perimeter: {}", rectangle_copy.area());
    println!("Parameters of rectangle are valid: {}", rectangle.verify_parameters());

    let circle = Shape::create_shape("circle", vec![5.0]).unwrap();
    match circle {
        Shape::Circle(radius) => println!("Circle radius: {}", radius),
        _ => (),
    }
    println!("Area of circle: {}", circle.area());
    println!("Perimeter of circle: {}", circle.perimeter());
    let mut circle_copy = circle.clone();
    circle_copy.double_perimeter();
    println!("Doubled perimeter of circle: {}", circle_copy.perimeter());
    println!("Area with doubled perimeter: {}", circle_copy.area());
    println!("Parameters of circle are valid: {}", circle.verify_parameters());
}



    

    



