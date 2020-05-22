pub fn loop_examples() {
    println!("Loop Examples");
    println!();

    // Declare a mutable int
    let mut x = 1;

    // For infinite loops, use 'loop' (no need for 'while(true)' or 'for(;;)' ugliness!)
    println!("loop");
    loop {
        if ((x % 2) == 0) {
            println!("{} is even", x);
        } else {
            println!("{} is even", x);
        }

        if (x == 10) {
            break;
        }

        // Increment 'x'. Note there is no C-style '++' operator
        x += 1;
    }

    // Standard 'while' loop stuff
    println!("while");
    x = 0;
    while (x <= 100) {
        print!("{} ", x);
        x += 2;
    }
    println!();

    println!("while loops with breaking to outer-while loop");
    let mut z = 0;
    x = 0;
    'outer_while: while (x < 10) {
        x = x + 1;
        println!("x = {}", x);
        while (z < 10) {
            z = z + 1;
            println!("\tz = {}", z);
            if (z == 5) {
                // 'break' would normally just jump out of the current 'while' loop, but we can
                // also break out of parent loops, and parent-parent loops, etc.! Much nicer than
                // having an boolean flag for breaking out of massively embedded loops. Also works
                // for standard 'for' loops
                break 'outer_while;

                // We can also 'continue' to the outer loop!
                //continue 'outer_while;
            }
        }
    }

    println!("for loops");
    println!("basic for loop");
    for x in 10..20 {
        print!("{} ", x);
    }
    println!();

    println!("enumerate on range with for loop");
    // Note we don't need to declare new variable 'y' with 'let' !
    for (x, y) in (10..20).enumerate() {
        // x will be the iteration (zero-indexed)
        // y will be the actual value
        println!("x={}, y={} ", x, y);
    }

    println!("looping with step of 2");
    for x in (1..10).step_by(2) {
        print!("{} ", x);
    }
    println!();
 
}