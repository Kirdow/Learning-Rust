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

    // reset count
    count = 1;

    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        // Inc
        count += 1
    }

    // For Range
    for x in 0..100 {
        if x % 7 == 2 {
            println!("{}", x);
        }
    }
}
