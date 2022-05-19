// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    const LEGAL_AGE: i32 = 18;
    let age = 16;

    let check_id = false;

    // If/Else
    if age >= LEGAL_AGE && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < LEGAL_AGE && check_id {
        println!("Bartender: Sorry bud, need to be {}. Come back in {} years!", LEGAL_AGE, LEGAL_AGE - age);
    } else {
        println!("Bartender: I'll need to see your ID");
    }
}
