pub fn array_examples() {
    println!("Array Examples");
    println!();

    // Arrays are fixed sized lists of the same type
    let some_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 
    println!("Whole array: {:?}", some_array);
    println!("0th element: {}", some_array[0]);
    println!("last element: {:?}", some_array.last()); // Returns Some() or None!!
    println!("Array length: {}", some_array.len());
    
    // Eurgh! Have to declare a new int just to look for it in the array rather than pass an integer literal?!?
    let x = 3;
    println!("Contains number 3? {}", some_array.contains(&x));
 
    // Slice an array by using a reference to it
    // :? formats the printing of the array
    println!("Range 3 to 6 : {:?}", &some_array[3..6]);
}