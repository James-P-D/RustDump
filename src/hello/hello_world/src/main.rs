use std::env;
//use std::os;

// CTRL+SHIFT+B to BUILD
// F5 to RUN

mod variables;


fn main() {
    println!("Hello, world!");

    // Display command-line arguments
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    variables::variable_examples();

    // Return zero to OS
    ::std::process::exit(0); 
}
