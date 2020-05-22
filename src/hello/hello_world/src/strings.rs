pub fn string_examples() {
    println!("String Examples");
    println!();

    let rand_string = "This is an example string";
 
    // String length
    println!("Length : {}", rand_string.len());
 
    // Split a string in half at index
    let (first, second) = rand_string.split_at(rand_string.len() / 2);
    println!("First : '{}' Second : '{}'", first, second);
 
    // We can easily split on whitespace
    println!("Whitespace");
    for s in rand_string.split_whitespace() {
        print!("'{}', ", s)
    }
    println!();

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars(); 
    let mut indiv_char = chars.next(); 
    println!("Chars");
    loop {
        // Pattern match like switch
        match indiv_char {
 
            // If show print
            Some(x) => print!("'{}', ", x), 
            // If None break
            None => break,
        }
        indiv_char = chars.next();
    }
    println!();

     // Iterate over lines of string
     let rand_string2 = "I am a random string\nThere are other strings like it\nThis string is the best"; 
     let mut lines = rand_string2.lines();  
     let mut indiv_line = lines.next();  
     println!("Lines");
     loop {
         match indiv_line {
             Some(x) => println!("Line: {}", x),
             None => break,
         }
         indiv_line = lines.next();
     }
     println!();

     // Find string in string
    println!("Does string contain 'best'? : {}", rand_string2.contains("best")); 
}