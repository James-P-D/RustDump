pub fn ownership_examples() {
    println!("Ownership Examples");
    println!();

    // There is only one binding for each resource
    // so if you assign data to another variable
    // the original can't access the data
    let vect1 = vec![1, 2, 3];
    println!("vect1[0] : {}", vect1[0]);
    let vect2 = vect1;

    // error: use of moved value: `vect1`
    // println!("vect1[0] : {}", vect1[0]);
    
    // But this will work fine
    println!("vect2[0] : {}", vect2[0]);
 
    // Primitive types can however copy values
    let mut prim_val = 1;
    let prim_val2 = prim_val;
    println!("prim_val: {}, prim_val2: {}", prim_val, prim_val2);
    let prim_val = 2; // This doesn't change the value of 'prim_val2'
    println!("prim_val: {}, prim_val2: {}", prim_val, prim_val2);
}