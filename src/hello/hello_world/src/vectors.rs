pub fn vector_examples() {
    println!("Vector Examples");
    println!();

    // Vectors are lists (unlike arrays, we can add/remove items)
    let mut some_vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 
    println!("Whole vector: {:?}", some_vector);
    println!("0th item : {}", some_vector[0]);
    println!("0th item : {}", some_vector[0]);
 
    println!("Iterate through items in vector");
    for i in &some_vector {
        print!("{} ", i);
    }
    println!();
 
    println!("Iterate through items in vector after pushing 99");
    some_vector.push(99);
    for i in &some_vector {
        print!("{} ", i);
    }
    println!();
 
    // Pop item from the end
    let popped_Val = some_vector.pop();
    println!("Popped value: {:?}", popped_Val); // Some() or None()
}