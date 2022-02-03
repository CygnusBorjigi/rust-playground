use std::collections::HashMap;
pub fn run() {
    let text = "this is somehting and that something is working";

    let mut hash_map = HashMap::new();

    for each in text.split_whitespace() {
        let count = hash_map.entry(each).or_insert(0);
        *count += 1
    }

    println!("{:?}", hash_map);
}