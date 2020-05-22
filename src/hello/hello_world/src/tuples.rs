pub fn tuple_examples() {
    println!("Tuple Examples");
    println!();
    
    // Tuples are fixed sized lists of many types
    let some_tuple = ("Foo", 123, 3.1415);
    println!("Display the whole tuple: {:?}", some_tuple);
    // Get value by index (zero-indexed obvs)
    println!("Stringy : {}", some_tuple.0);
    println!("Inty : {}", some_tuple.1);
    println!("Floaty : {}", some_tuple.2);

    println!("Get the individual items");
    let (a, b, c) = some_tuple;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    
    // You can also define the data types
    let some_typed_tuple: (f32, &str, i8) = (12.34, "abcdefg", 99);
    println!("Display the typed tuple: {:?}", some_typed_tuple);
    
}