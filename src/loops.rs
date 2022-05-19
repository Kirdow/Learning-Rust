// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop
    loop {
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }
}
