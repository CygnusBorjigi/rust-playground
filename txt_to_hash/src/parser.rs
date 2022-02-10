pub fn run(file_content: String) -> Vec<Element> {
    let char_list: Vec<char> = explode(file_content);
    let first_pass = char_whitespace_linebreak(char_list);
    let second_pass = char_to_word(first_pass);
    return second_pass;
}

#[derive(Debug)]
pub enum Element {
    Word(String),
    Char(char),
    Punctuation(char),
    Whitespace(i32),
    Linebreak,
}

fn explode (source: String) -> Vec<char> {
    return source.chars().collect();
}

fn char_whitespace_linebreak (source: Vec<char>) -> Vec<Element> {
    let iter = source.iter();
    let output: Vec<Element> = iter.map(|each| {
        if each == &' ' {
            Element::Whitespace(1)
        } else if each == &'\n' {
            Element::Linebreak
        } else if is_punctuation(each) {
            Element::Punctuation(*each)
        } else {
            Element::Char(*each)
        }
    }).collect();
    return output;
}

fn is_punctuation (source: &char) -> bool {
    let target = ",.:;-'";
    return target.contains(*source);
}

fn char_to_word (source: Vec<Element>) -> Vec<Element> {
    let mut result = Vec::new();
    let mut accum = Vec::new();
    for i in source {
        match i {
            Element::Char(ele) => accum.push(ele),
            Element::Punctuation(ele) => {
                if &accum.len() != &0 {
                    let mut new_word = String::from("");
                    for a in &accum {
                        new_word.push(*a);
                    }
                    result.push(Element::Word(new_word));
                    accum = Vec::new();
                } 
                result.push(Element::Punctuation(ele));
            },
            Element::Whitespace(ele) => {
                if &accum.len() != &0 {
                    let mut new_word = String::from("");
                    for a in &accum {
                        new_word.push(*a);
                    }
                    result.push(Element::Word(new_word));
                    accum = Vec::new();
                } 
                result.push(Element::Whitespace(ele));
            },
            Element::Linebreak => {
                if &accum.len() != &0 {
                    let mut new_word = String::from("");
                    for a in &accum {
                        new_word.push(*a);
                    }
                    result.push(Element::Word(new_word));
                    accum = Vec::new();
                } 
                result.push(Element::Linebreak);
            },
            Element::Word(ele) => result.push(Element::Word(ele))
        }
    }
    if &accum.len() != &0 {
        let mut new_word = String::from("");
        for a in &accum {
            new_word.push(*a);
        }
        result.push(Element::Word(new_word));
    }
    return result;
}
