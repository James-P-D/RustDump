use std::{u8};

pub fn output_examples() {
    println!("Output Examples");
    println!();

    let some_float: f64 = 123.456;

    // Format output
    println!("We can infer parameter order. 1st param = {} 2nd param = {}", 99, "hello");
    println!("Or we can specify the parameter by number. 1st param = {0} 2nd param = {1} 1st again = {0}", 99, "hello");
    println!("temp_int = {temp_int}", temp_int = -5);
    
    // We can specify decimal places
    println!("some_float to 2 d.p. = {:.2}", some_float);

    let some_byte: u8 = 255;
    println!("some_byte: {}", some_byte);
    println!("Bin: {:b}", some_byte);
    println!("Hex: {:x}", some_byte);
    println!("Oct: {:o}", some_byte);

    println!("leading spaces:{ten:>15}", ten = 10);
}