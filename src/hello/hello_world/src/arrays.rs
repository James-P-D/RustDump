pub fn array_examples() {
    println!("Array Examples");
    println!();

    // Arrays are fixed sized lists of the same type
    let some_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
 
    println!("0th element: {}", some_array[0]);
    println!("Array length: {}", some_array.len());
    println!("Contains number 3? ", some_array.contains(&{3}));
 
    // Slice an array by using a reference to it
    // :? formats the printing of the array
    println!("Range 3 to 6 : {:?}", &some_array[3..6]);    
}