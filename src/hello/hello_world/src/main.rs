use std::env;
//use std::os;

// CTRL+SHIFT+B to BUILD
// F5 to RUN

mod variables;
mod output;
mod maths;
mod conditionals;
mod loops;
mod strings;
mod input;
mod arrays;

fn main() {
    // Display command-line arguments
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    //variables::variable_examples();
    //output::output_examples();
    //maths::maths_examples();
    //conditionals::conditional_examples();
    //loops::loop_examples();
    //strings::string_examples();
    //input::input_examples();
    arrays::array_examples();

    // Return zero to OS
    ::std::process::exit(0); 
}
