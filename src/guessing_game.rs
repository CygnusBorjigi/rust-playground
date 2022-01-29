use std::io; // For accepting user input
use std::cmp::Ordering; // For compare numbers and show result
use rand::Rng; // For generating random number
use colored::*; // For coloring the text output

pub fn run_game() {
    println!("u guess a number");
    let secret = rand::thread_rng().gen_range(1, 101);    
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read line failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter an number");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "The actual number is larger".red()),
            Ordering::Greater => println!("{}", "The actual number is smaller".red()),
            Ordering::Equal => {
                println!("{}", "You got it".green());
                break;
            },
        };
    }
}
