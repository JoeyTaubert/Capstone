use std::io::stdin;

mod analysis;
mod cap;
mod ml;

fn main() {
    let mut flag = true;

    while flag {
        println!("\nPlease choose a function to execute:");
        println!("0: Exit");
        println!("1: Capture");
        println!("2: Analysis");
        println!("3: ML");

        let mut input = String::new();
        println!("Enter your choice (1, 2, or 3):");
        stdin().read_line(&mut input).expect("Failed to read line");
        match input.as_str().trim() {
            "0" => flag = false,
            "1" => (), //cap::main(), // need to spawn new async task to handle this
            "2" => (), //analysis::main(),
            "3" => ml::main(),
            _ => println!("\nInvalid input.\n"),
        };
    }
}
