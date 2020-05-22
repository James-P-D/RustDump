struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
 
fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

// It is recommended to define struct methods with impl
impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

pub fn struct_examples() {
    println!("Struct Examples");
    println!();

    // Create a mutable circle
    let mut circle1 = Circle {
        x: 10.0, y: 10.0, radius: 10.0
    };
 
    // Get Circle values
    println!("X : {} Y : {} R : {}", circle1.x, circle1.y, circle1.radius);
 
     // Define a function to operate on the struct
     println!("Circle Radius : {}", get_radius(&circle1));
 
     // It is recommended to create struct methods with impl
     println!("Circle X : {}", circle1.get_x());
}