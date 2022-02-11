pub use crate::parser::Element;

pub fn run(inter_rep: Vec<Element>) -> Hashmap {
    let mut hash_map: Hashmap = Hashmap::create_map();
    hash_map.add_word(String::from("Something"));
    let to_change = String::from("Something");
    hash_map.add_word_count(&to_change);
    hash_map.add_word_count(&to_change);
    return hash_map;
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

    fn check_for_word (&self, target: &String) -> bool{
        let iter = self.Words.iter();
        for i in iter {
            if &i.0 == target {
                return true
            }
        }
        return false
    }

    fn add_word (&mut self, target: String) {
        self.Words.push((target, 1))
    }

    fn add_word_count (&mut self, target: &String) {
        for mut i in self.Words.iter_mut() {
            if i.0 == *target {
                i.1 += 1;
            }
        }
    }
}

