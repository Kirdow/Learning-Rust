// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    const LEGAL_AGE: i32 = 18;
    let age = 16;

    // If/Else
    if age >= LEGAL_AGE {
        println!("Bartender: What would you like to drink?");
    } else {
        println!("Bartender: Sorry bud, need to be {}. Come back in {} years!", LEGAL_AGE, LEGAL_AGE - age);
    }
}
