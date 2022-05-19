/*
Primitive Types--

Integer Types:
unsigned    u8      u16     u32     u64     u128
signed      i8      i16     i32     i64     i128

Floating Point Types:
signed                      f32     f64

Other Types:
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Hint: Rust is statically typed but compiler may infer the type based on context

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4040404040404040;

    // Find max size
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max u128: {}", std::u128::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression (inferred)
    let is_greater = 10 > 5;

    // Character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, face));
}
