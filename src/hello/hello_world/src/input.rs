use std::io::stdin;

pub fn input_examples() {
    println!("Input Examples");
    println!();

    println!("Enter a string: ");
    let mut some_string = String::new(); 
    // Pass the reference where we store the string
    // entered on the keyboard
    let input = stdin().read_line(&mut some_string);
    println!("You entered the string: '{}'", some_string);
    println!();

    // Read another string
    println!("Enter an unsigned 32-bit int: ");
    let mut some_string2 = String::new(); 
    stdin().read_line(&mut some_string2).expect("failed to read from stdin");
    let trimmed = some_string2.trim();
    // Attempt to convert to unsigned 32-bit int
    match trimmed.parse::<u32>() {
        Ok(i) => println!("Your integer input: {}", i),
        Err(..) => println!("This was not an integer: {}", trimmed),
    };
    println!();

}