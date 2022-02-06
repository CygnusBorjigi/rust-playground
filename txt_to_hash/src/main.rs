use std::env;
use std::fs;

fn main() {
    // take and check user input
    let input_args: Vec<String> = env::args().collect();
    if input_args.len() < 2 {
        eprintln!("please enter the file name");
    } else if input_args.len() > 2 {
        eprintln!("only one argument is accepted");
    };

    // search and open file
    let file_content = fs::read_to_string(&input_args[1]).expect("file cannot be open");
    println!("{:?}", file_content);

    // parse the file into indivisual words
    let file_char_list = explode(file_content);
    println!("{}", file_char_list.len());
    println!("{:?}", file_char_list);
    
}

fn explode (source: String) -> Vec<char> {
    return source.chars().collect();
}
