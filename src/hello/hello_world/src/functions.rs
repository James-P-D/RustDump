pub fn function_examples() {
    println!("Function Examples");
    println!();

    // Call function with parameter but no return value
    display_name("John Doe");
    
    let x = 10;
    println!("Sum of 5 and 10 = {}", sum_two_numbers(5, x));

    // Functions are call-by-value by default. Changes to 'y' made in 'try_to_increment()'
    // won't be reflected when we get back to 'function_examples()'
    let mut y = 5;
    println!("y before calling function: {}", y);
    try_to_increment(y);
    println!("y after calling function: {}", y);
    println!();

    let mut z = 5;
    println!("z before calling function: {}", z);
    try_to_increment_again(&mut z); // Pass mutable reference
    println!("z after calling function: {}", z);    
    println!();

    // We can't really pass a variable number of arguments in Rust (no C-style ellipsis '...' operator, or
    // the horrors of '*args and **kwargs' in Python.) Instead we have to make do with passing an array.
    let total1 = sum_numbers(&[1, 2, 3]);
    println!("Sum of three numbers = {}", total1);
    let total2 = sum_numbers(&[1, 2, 3, 4, 5, 6]);
    println!("Sum of six numbers = {}", total2);
}

// Note we *always* have to specify datatypes for parameters
fn display_name(name: &str) { // Functions have implicit void return type
    println!("Hello {}", name);
}

fn sum_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

fn try_to_increment(mut a: i32) {
    println!("a before incrementing: {}", a);
    a = a + 1;
    println!("a after incrementing: {}", a);
}

fn try_to_increment_again(b: &mut i32) {
    println!("b before incrementing: {}", b);
    *b += 1;
    println!("b after incrementing: {}", b);
}

fn sum_numbers(args: &[i32]) -> i32 {
    let mut total :i32 = 0;
    for arg in args {
        total += *arg;
    }
    return total;
}
