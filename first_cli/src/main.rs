use std::env;
use std::process; // for exit the program without panic
use first_cli::Config;

fn main() {
     let user_args: Vec<String> = env::args().collect(); // collect user input
     let parsed_args = Config::new(&user_args).unwrap_or_else(|err| {
         eprintln!("Problem occur when parsing input: {}", err);
         process::exit(1);
     });

     if let Err(e) = first_cli::run(parsed_args){
         eprintln!("Error occured: {}", e);
         process::exit(1);
     }
}

