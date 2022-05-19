pub fn run() {
    // Print to console
    println!("This is run from print.rs!");

    // Basic formatting
    println!("{}, {} is from {}", "Kirdow", 24, "Sweden");

    // Positional arguments
    println!("{0} is from {1} and {0} is {2} yo", "Kirdow", "Sweden", 24);

    // Named arguments
    println!("{name} likes to play {activity}", name = "Kirdow", activity = "Video Games");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder traits + Positional arguments
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}
