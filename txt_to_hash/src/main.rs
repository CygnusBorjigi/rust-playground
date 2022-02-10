use std::env;
use crate::hasher::Hashmap;
mod user_input;
mod parser;
mod hasher;

fn main() {
    let file_content: String = user_input::run(env::args().collect());
    let internal = parser::run(file_content);
    let hash_map: Hashmap = hasher::run(internal);
    println!("{:?}", hash_map);
}



