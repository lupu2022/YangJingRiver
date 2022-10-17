use std::collections::HashMap;

use crate::TNT;
use crate::stack::{YjrStack, YjrHash};

#[derive(Debug, PartialEq, Clone, Copy)]
enum WordByte {
    Number(TNT),
    Symbol(usize),
    Native(usize),
    User(usize),
}
type UserWord = Vec<WordByte>;

pub struct YjrEnviroment {
    user_words :    Vec<UserWord>,
    user_words_map: HashMap<String, usize>,

    native_words :  Vec< fn(&mut YjrStack) >,
    native_words_map: HashMap<String, usize>,

    string_library: Vec< String >,
}

pub struct YjrRuntime {
    stack:      YjrStack,
    hash:       Vec<YjrHash>,
    binary:     UserWord,
}

impl YjrEnviroment {
    pub fn new() -> Self {
        let mut ret = YjrEnviroment {
            user_words:     Vec::new(),
            user_words_map: HashMap::new(),
            native_words:   Vec::new(),
            native_words_map: HashMap::new(),
            string_library: Vec::new(),
        };
        ret.string_library.push("".to_string());
        ret
    }

    fn insert_user_word(&mut self, name: &str, word: UserWord) {
        if self.user_words_map.get(name).is_some() {
            panic!("Can't define word with same name");
        }

        let n = self.user_words.len();
        self.user_words_map.insert(name.to_string(), n);
        self.user_words.push( word );
    }

    fn get_string_index(&mut self, s: &str) -> usize {
        for i in 0..self.string_library.len() {
            if self.string_library[i].as_str() == s {
                return i
            }
        }
        let ret = self.string_library.len();
        self.string_library.push(s.to_string());
        ret
    }

    fn compile(&mut self, code: &str) -> UserWord {
        fn tokenize(expr: &str) -> Vec<String> {
            expr.replace("\n", " ")
                .replace("{", " ")
                .replace("}", " ")
                .replace("(", " ")
                .replace(")", " ")
                .replace("]", " ] ")
                .replace("[", " [ ")
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        }

        fn remove_comment(code: &str) -> String {
            let mut contents = "".to_string();
            for line in code.lines() {
                let mut line = line.to_string();
                if let Some(pos) = line.find(";") {
                    let (code, _) = line.split_at(pos);
                    line = code.to_string();
                }

                contents.push_str( &line );
                contents.push_str("\n");
            }
            contents
        }

        fn check_symbol(symbol: &str) -> bool {
            symbol.chars().all( |c| {
                (c == '_') || matches!(c, 'a'..='z') || matches!(c, 'A'..='Z') || matches!(c, '0'..='9')
            })
        }

        fn check_number(token: &str) -> Option<TNT> {
            let dv = token.parse::<TNT>();
            if dv.is_ok() {
                return Some( dv.unwrap());
            }
            return None;
        }

        let tokens = tokenize( &remove_comment(code) );

        let mut main_code: UserWord = Vec::new();
        let mut word_code: Option<UserWord> = None;
        let mut loop_code: Option<UserWord> = None;
        let mut list_count: Option<usize> = None;

        for token in &tokens {
            let token = token.as_str();

            // first pass, processing command primitive
            let mut new_byte: WordByte = if token == "#def" {
                if !word_code.is_none() {
                    panic!("Can't define new word inside a word.");
                }
                if !loop_code.is_none() {
                    panic!("Can't define in a loop macro.");
                }
                if !list_count.is_none() {
                    panic!("Can't define new word in a list macro.");
                }
                word_code = Some( Vec::new() );
                continue;
            } else if token == "#loop" {
                if !loop_code.is_none() || !list_count.is_none() {
                    panic!("Can't define loop/list macro inside another loop/list macro");
                }
                loop_code = Some( Vec::new() );
                continue;
            } else if token == "#end" {
                if !list_count.is_none() {
                    panic!("Can't ending a word  or a loop in a list macro.");
                }

                if let Some(ref w) = loop_code {
                    // loop section ending
                    if w.len() == 0 {
                        panic!("#loop macro without loop number!");
                    }
                    if let WordByte::Number(ln) = w[0] {
                        if ln.fract() != 0.0 {
                            panic!("Loop count must be a integer");
                        }
                        for _ in 0.. (ln as usize) {
                            for i in 1..w.len() {
                                main_code.push( w[i] );
                            }
                        }
                    } else {
                        panic!("First item must be a number in #loop");
                    }

                    loop_code = None;
                    continue;
                }

                if let Some(ref mut w) = word_code {
                    if w.len() == 0 {
                        panic!("#define macro without word name!");
                    }
                    if let WordByte::Symbol(s) = w[0] {
                        let s = self.string_library[s].clone();
                        if s.starts_with("$") {
                            panic!("Word's name can't begin with $");
                        }
                        let mut new_word : UserWord = Vec::new();
                        for i in 1..w.len() {
                            new_word.push( w[i] );
                        }
                        self.insert_user_word(&s, new_word);
                    } else {
                        panic!("First item must be a number in #loop");
                    }
                    word_code = None;
                    continue;
                }

                panic!("Find #end without any begin primitive");
            } else if token == "[" {
                if !loop_code.is_none() || !list_count.is_none() {
                    panic!("Can't define loop/list macro inside another loop/list macro");
                }
                if let Some(ref w) = word_code {
                    if w.len() == 0 {
                        panic!("First item of a word must a symbol named this word!");
                    }
                }
                list_count = Some(0);
                continue;
            } else if token == "]" {
                if let Some(ln) = list_count {
                    list_count = None;
                    WordByte::Number( ln as TNT )
                } else {
                    panic!("']' list macro appears without begin '['");
                }
            } else {
                // first item of word is name.
                if let Some(ref mut w) = word_code {
                    if w.len() == 0 {
                        if check_symbol(&token) {
                            let i = self.get_string_index(token);
                            w.push( WordByte::Symbol(i) );
                            continue;
                        } else {
                            panic!("Word name must be a alphabetnumberic");
                        }
                    }
                }

                // do some translate in second pass
                if let Some(n) = check_number( token) {
                    WordByte::Number(n)
                } else {
                    let i = self.get_string_index(token);
                    WordByte::Symbol(i)
                }
            };

            // second pass: translate symbol to native or user word.
            let mut push_byte = |x: WordByte| {
                if let Some(ref mut uw) = loop_code {
                    uw.push(x);
                    return;
                }
                if let Some(ref mut uw) = word_code {
                    uw.push(x);
                    return;
                }
                main_code.push(x);
            };

            let mut symbol = match new_byte {
                WordByte::Number(_) => {
                    push_byte(new_byte);
                    continue;
                },
                WordByte::Symbol(n) => {
                    self.string_library[n].clone()
                },
                _ => {
                    panic!("new_byte must a symbol or number after first pass")
                }
            };

            // checking is a keyword
            if symbol == "true" {
                new_byte = WordByte::Number(1.0);
                push_byte(new_byte);
                continue;
            }
            if symbol == "false" {
                new_byte = WordByte::Number(0.0);
                push_byte(new_byte);
                continue;
            }
            if symbol == "null" {
                new_byte = WordByte::Symbol(0);
                push_byte(new_byte);
                continue;
            }

            // checking is a native word
            if let Some(n) = self.native_words_map.get(&symbol) {
                new_byte = WordByte::Native(*n);
                push_byte(new_byte);
                continue;
            }

            // checking is a user word
            if let Some(n) = self.user_words_map.get(&symbol) {
                new_byte = WordByte::User(*n);
                push_byte(new_byte);
                continue;
            }

            // checking is a valid symbol
            if symbol.starts_with("$") {
                symbol.remove(0);
                if !check_symbol( &symbol ) {
                    panic!("Symbol must include alphanumbric or '_'");
                }
                push_byte(new_byte);
                continue;
            }

            panic!("An symbol can't bind to user/native word!");
        }

        if !word_code.is_none() || !loop_code.is_none() || !list_count.is_none() {
            panic!("#loop list or word without ending");
        }

        main_code
    }

    pub fn build(&self, code: &str ) -> YjrRuntime {
        todo!()
    }

    pub fn run(&mut self, runtime: &mut YjrRuntime) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::YjrEnviroment;

    #[test]
    fn simple_test() {
        let mut env = YjrEnviroment::new();
        let code = "3.14 'kaka' goto";
        env.compile(code);
    }
}



