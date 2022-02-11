pub use crate::parser::Element;

pub fn run(inter_rep: Vec<Element>) -> Hashmap {
    let mut hash_map: Hashmap = Hashmap::create_map();
    hash_map.build_map(inter_rep);
    return hash_map;
}

#[derive(Debug)]
pub struct Hashmap {
    words: Vec<(String, i64)>,
    puncutation: Vec<(char, i64)>,
    whitespace: i64,
    linebreak: i64,
}

impl Hashmap {
    fn build_map (&mut self, source: Vec<Element>){
        for i in source{
            match i {
                Element::Word(ele) => {
                    if Self::check_for_word(self, &ele) {
                        Self::add_word_count(self, &ele);
                    } else {
                        Self::add_word(self, ele);
                    }
                },
                Element::Punctuation(ele) => {
                    if Self::check_for_pun(self, &ele) {
                        Self::add_pun_count(self, &ele);
                    } else {
                        Self::add_pun(self, ele);
                    }
                },
                Element::Char(_) => {
                    eprintln!("internal parsing error");
                },
                Element::Whitespace(_) => {
                    Self::add_whitespace(self);
                },
                Element::Linebreak => {
                    Self::add_linebreak(self);
                },
            }
        }
    }
}

impl Hashmap {
    fn create_map () -> Hashmap {
        Hashmap {
            words: [].to_vec(),
            puncutation: [].to_vec(),
            whitespace: 0,
            linebreak: 0,
        }
    }

    fn check_for_word (&self, target: &String) -> bool{
        let iter = self.words.iter();
        for i in iter {
            if &i.0 == target {
                return true
            }
        }
        return false
    }

    fn add_word (&mut self, target: String) {
        self.words.push((target, 1));
    }

    fn add_word_count (&mut self, target: &String) {
        for mut i in self.words.iter_mut() {
            if i.0 == *target {
                i.1 += 1;
            }
        }
    }

    fn check_for_pun (&self, target: &char) -> bool {
        let iter = self.puncutation.iter();
        for i in iter {
            if &i.0 == target {
                return true
            }
        }
        return false
    }

    fn add_pun (&mut self, target: char) {
        self.puncutation.push((target, 1));
    }

    fn add_pun_count (&mut self, target: &char) {
        for mut i in self.puncutation.iter_mut() {
            if i.0 == *target {
                i.1 += 1;
            }
        }
    }

    fn add_whitespace (&mut self) {
        self.whitespace += 1;
    }
    fn add_linebreak (&mut self) {
        self.linebreak += 1;
    }
}

