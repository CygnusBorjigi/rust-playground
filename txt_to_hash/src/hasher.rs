pub use crate::parser::Element;

pub fn run(inter_rep: Vec<Element>) -> Hashmap {
    return Hashmap::create_map();
}

#[derive(Debug)]
pub struct Hashmap {
    Words: Vec<(String, i64)>,
    Puncutation: Vec<(char, i64)>,
    Whitespace: i64,
    Linebreak: i64,
}

impl Hashmap {
    fn create_map () -> Hashmap {
        Hashmap {
            Words: [].to_vec(),
            Puncutation: [].to_vec(),
            Whitespace: 0,
            Linebreak: 0,
        }
    }
}
