fn main(){
    use std::f64::consts::PI;

    // Define a trait for regular polygons
    trait RegularPolygon {
        fn perimeter(&self) -> f64;
        fn area(&self) -> f64;
        fn radius(&self) -> f64;
        fn apothem(&self) -> f64;
    }
    
    // Define a struct for regular polygons
    struct Polygon {
        sides: usize,
        length: f64,
    }
    
    impl Polygon {
        fn new(sides: usize, length: f64) -> Self {
            Self { sides, length }
        }
    }
    
    impl RegularPolygon for Polygon {
        fn perimeter(&self) -> f64 {
            self.sides as f64 * self.length
        }
    
        fn area(&self) -> f64 {
            0.5 * self.perimeter() * self.apothem()
        }
    
        fn radius(&self) -> f64 {
            self.length / (2.0 * PI / self.sides as f64).sin()
        }
    
        fn apothem(&self) -> f64 {
            self.length / (2.0 * PI / self.sides as f64).tan()
        }
    }
    
   
        let radius_lengths = vec![1.0, 2.0, 3.0]; //alternating between different lengths for the sides 
    
        let num_sides = vec![6, 12, 24, 128, 256, 512, 1024, 2048, 65536]; //defing the number of sides for the pologon 
    
        for sides in &num_sides {
            for &radius in &radius_lengths {
                let polygon = Polygon::new(*sides, radius);
                let inscribed_circle_radius = polygon.apothem();
                let circumscribed_circle_radius = polygon.radius();
                
                // Calculating areas for both circles
                let inscribed_circle_area = PI * inscribed_circle_radius * inscribed_circle_radius;
                let circumscribed_circle_area = PI * circumscribed_circle_radius * circumscribed_circle_radius;
                
                let polygon_area = polygon.area();
                
                // Calculate differences in areas
                let inscribed_circle_difference = polygon_area - inscribed_circle_area;
                let circumscribed_circle_difference = polygon_area - circumscribed_circle_area;
                
                println!("Polygon with {} sides and radius {}: Perimeter = {:.4}, Area = {:.4}, Apothem = {:.4}, Radius = {:.4}", 
                         *sides, radius, polygon.perimeter(), polygon.area(), polygon.apothem(), polygon.radius());
                println!("Inscribed Circle: Radius = {:.4}, Area = {:.4}, Difference from Polygon Area (Polygon- Inscribed Circle) = {:.4}", 
                         inscribed_circle_radius, inscribed_circle_area, inscribed_circle_difference);
                println!("Circumscribed Circle: Radius = {:.4}, Area = {:.4}, Difference from Polygon Area (Polygon- Circumscribed Circle) = {:.4}", 
                         circumscribed_circle_radius, circumscribed_circle_area, circumscribed_circle_difference);
                println!("------------------------------------------------------");
            }
        }
    }
    

    

    
    
    