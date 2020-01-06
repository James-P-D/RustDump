use std::env;
//use std::os;

// CTRL+SHIFT+B to BUILD
// F5 to RUN

fn main() {
    println!("Hello, world   Is it working!!!!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    ::std::process::exit(1); 
}
