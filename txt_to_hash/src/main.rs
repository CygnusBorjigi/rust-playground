use std::env;
mod user_input;
mod parser;

fn main() {
    let file_content: String = user_input::run(env::args().collect());
    let internal = parser::run(file_content);
    println!("{:?}", internal);
}

#[derive(Debug)]
struct HashMap {
    word: Vec<(String, i32)>,
    punctuation: Vec<(char, i32)>,
    whitespace: i32,
    linebreak: i32,
}


