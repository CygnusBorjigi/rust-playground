use std::fs;

pub fn run(user_input: Vec<String>) -> String{
    if user_input.len() < 2 {
        eprintln!("please enter the file name");
    } else if user_input.len() > 2 {
        eprintln!("Only one argument is expected");
    };
    let file_name: &String = &user_input[1];
    let file_content: String = fs::read_to_string(file_name).expect("Error occured when trying to read the file");
    return file_content;
}

