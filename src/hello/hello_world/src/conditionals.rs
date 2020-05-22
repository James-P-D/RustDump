pub fn conditional_examples() {
    println!("Conditional Examples");
    println!();

    let age_old = 6;

    // 'if' statements are pretty standard.. Supports all the usual !=, >=, ||, && etc..
    // Only important thing to remember is that curly-braces are mandatory! Even if
    // we only have 1 statement (unlike C!)
    if (age_old == 5) {
        println!("Go to kindergarten");
    } else if (age_old > 5) && (age_old <= 18){
        println!("Go to grade {}", (age_old - 5));
    } else if (age_old <= 25) && (age_old > 18) {
        println!("Go to college");
    } else {
        println!("Do what you want");
    }
 
    // Ternary operator is Python-style ('if..else' rather than '?:' in C, C# etc.)
    // Again, curly braces are mandatory
    let x = 5;
    let y = 10;
    let max = if (x > y) {x} else {y};
    println!("x = {}, y = {}, max = {}", x, y, max);

    // Pattern matching instead of switch/case statements..
    let num = 7;
    match num {
        // Check for zero
        0 => println!("Zero"), 

        // Check for a number of different values
        2 | 3 | 5 | 7 => {
            // Note the braces required for multiple statements
            println!("Prime!");
            println!("Although we only check for small numbers")
        }
        
        // Check for a range
        13..=19 => println!("Teens"),

        // Anonymous variable '_' instead of 'default'
        _ => println!("Not zero, teens or a small prime")
    }
}