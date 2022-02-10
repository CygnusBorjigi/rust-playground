use std::env;
mod user_input;

fn main() {
    let file_content: String = user_input::run(env::args().collect());

    // parse the file into indivisual elements
    let file_char_list = explode(file_content);

    let first_pass =  identify_char_whitespace_linebreak(file_char_list);
    let second_pass = char_to_word(first_pass);
    println!("{:?}", second_pass);
}

#[derive(Debug)]
struct HashMap {
    word: Vec<(String, i32)>,
    punctuation: Vec<(char, i32)>,
    whitespace: i32,
    linebreak: i32,
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
            Element::WhiteSpace(ele) => {
                if &accum.len() != &0 {
                    let mut new_word = String::from("");
                    for a in &accum {
                        new_word.push(*a);
                    }
                    result.push(Element::Word(new_word));
                    accum = Vec::new();
                } 
                result.push(Element::WhiteSpace(ele));
            },
            Element::LineBreak => {
                if &accum.len() != &0 {
                    let mut new_word = String::from("");
                    for a in &accum {
                        new_word.push(*a);
                    }
                    result.push(Element::Word(new_word));
                    accum = Vec::new();
                } 
                result.push(Element::LineBreak);
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

fn identify_char_whitespace_linebreak (source: Vec<char>) -> Vec<Element> {
    let iter = source.iter();
    let output: Vec<_> = iter.map(|each| {
        if each == &' ' {
            Element::WhiteSpace(1)
        } else if each == &'\n' {
            Element::LineBreak
        } else if is_poun(*each) {
            Element::Punctuation(*each)
        } else {
            Element::Char(*each)
        }
    }).collect();
    return output;
}

fn is_poun (source: char) -> bool {
    let target = ",.:;-'";
    return target.contains(source);
}

fn explode (source: String) -> Vec<char> {
    return source.chars().collect();
}

#[derive(Debug)]
enum Element {
    Word(String),
    Char(char),
    Punctuation(char),
    WhiteSpace(i32),
    LineBreak,
}



