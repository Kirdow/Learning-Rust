// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColor(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }
}

pub fn run() {
    let mut c = Color { red: 255, green: 0, blue: 0 };

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    
    c.blue = 255;
    c.red = 120;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);

    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    tc.2 = 255;
    tc.0 = 120;

    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    let person = Person::new("John", "Doe");
    println!("Person {} {}", person.first_name, person.last_name);
}
