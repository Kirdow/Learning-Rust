// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Primitive str
    let my_str = "My Str";

    println!("Primitive [my_str] = {}", my_str);

    // Mutable str
    let hello = String::from("Hello");

    println!("Mutable String [hello] = {}", hello);

    // Get length
    println!("Length: {}", hello.len());
}
