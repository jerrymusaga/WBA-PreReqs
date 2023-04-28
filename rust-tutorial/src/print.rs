pub fn run(){
    // print to the console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("{} is from {}", "Jerry", "Mars");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Jerry", "Mars", "code");

    //Named arguments
    println!("{name} likes to {activity}", name="Jerry", activity="play football");

    // Placeholder traits
    println!("Binary: {:b} Hex:{:x} Octal:{:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (1, true, 0.2));

    //basic math
    println!("10+10={}", 10+10);
}