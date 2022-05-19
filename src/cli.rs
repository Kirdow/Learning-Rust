use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter a command!");
        return;
    }

    let command = args[1].clone();
    exec(command);
}

fn exec(cmd: String) {
    let result = match cmd.as_str() {
        "hello" => "Well hello there!",
        _ => "Sorry, who are you?"
    };

    println!("{}", result);
}