struct Square {
    size: f64,
}

struct Rectangle {
    height: f64,
    width: f64,
}
 
// Define the trait which is like an interface
trait HasArea {
    fn area(&self) -> f64;
}
 
// Now we can implement the HasArea interface for both structs
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.size * self.size
    }
}
 
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
 
pub fn trait_examples() {
    // Traits defines functionality that a type provides
    println!("Trait Examples");
    println!();

    let mut square = Square { size: 10.0 };
    let mut rectangle = Rectangle {width: 5.0, height: 12.0};

    println!("Square area = {}", square.area());
    println!("Rectangle area = {}", rectangle.area());

}