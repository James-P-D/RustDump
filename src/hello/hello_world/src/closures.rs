pub fn closure_examples() {
    println!("Closure Examples");
    println!();

    // Closures represent blocks of code and can
    // except parameters and be passed to functions
     
    // Note we *have* to specify types
    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7,8));
 
    // We can access variables outside the closure
    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));
}