use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

pub fn variable_examples() {
    println!("Variable Examples");
    println!();

    // let defines a variable
    // The data type will be guessed if not provided
    // Variable values are immutable (Can't change)
    let num = 10;
    
    // This line would produce an error since 'num' isn't mutable
    //num = num * 2;

    // Define a 32 bit mutable integer
    let mut age: i32 = 40;
 
    // We can also infer the datatype, in this case a signed 32-bit integer
    let some_int = 999i32;
    

    // There are many number types i8, i16, i32,
    // i64, u8, u16, u32, u64, isize, usize, f32, f64
 
    println!("Max i8 {}", i8::MAX);
    println!("Min i8 {}", i8::MIN);
    println!("Max i16 {}", i16::MAX);
    println!("Min i16 {}", i16::MIN);
    println!("Max i32 {}", i32::MAX);
    println!("Min i32 {}", i32::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Min i64 {}", i64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Min isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Min usize {}", usize::MIN);
    println!("Max f32 {}", f32::MAX);
    println!("Min f32 {}", f32::MIN);
    println!("Max f64 {}", f64::MAX);
    println!("Min f64 {}", f64::MIN);
 
    // There are booleans
    let is_it_true: bool = true;
 
    // Characters
    let let_x: char = 'x';
 
    // Place variable values in output
    println!("I am {} years old", age);
 
    // You can define multiple variables
    let (f_name, l_name) = ("Derek", "Banas");
    
    println!("I am {} years old and my name is {} {}", age, f_name, l_name);

    println!("My name is {1} {2} and I am {0} years old", age, f_name, l_name);
}