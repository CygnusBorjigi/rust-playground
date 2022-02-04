use std::env;
use std::fs;
use std::process; // for exit the program without panic
use std::error::Error; // for construction of the error type

fn main() {
     let user_args: Vec<String> = env::args().collect(); // collect user input
     let parsed_args = Config::new(&user_args).unwrap_or_else(|err| {
         println!("Problem occur when parsing input: {}", err);
         process::exit(1);
     });

     if let Err(e) = run(parsed_args){
         println!("Error occured: {}", e);
         process::exit(1);
     }
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new( args: &[String] ) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("there isn't enough argument...");
        }
         let query = args[1].clone();
         let file_name = args[2].clone();

         return Ok(Config{ query, file_name });
    }
}

fn run ( config: Config ) -> Result<(), Box<dyn Error>>{
     let content = fs::read_to_string(&config.file_name)?;
     println!("The query is: {}", &config.query);
     println!("The file name is: {}", &config.file_name);
     println!("{}", &content);
     return Ok(());
}
