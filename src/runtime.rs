use std::collections::HashMap;

use crate::TNT;
use crate::stack::{YjrStack, YjrHash};
use crate::builtin;

#[derive(Debug, PartialEq, Clone)]
enum WordCode {
    Number(TNT),
    Symbol(String),
    Native(String),
    User(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum WordByte {
    Number(TNT),
    Symbol(usize),
    Native(usize),
    User(usize),
}

type UserWord = Vec<WordCode>;
type UserBinary = Vec<WordByte>;

pub trait NativeWord  {
    fn boot(&mut self, stack: &mut YjrStack, local: &mut YjrHash, global: &mut YjrHash);
    fn tick(&mut self, stack: &mut YjrStack);
}

pub struct YjrEnviroment {
    user_words: HashMap<String, UserWord >,
    native_words: HashMap<String, fn()->Box<dyn NativeWord> >
}

pub struct YjrRuntime {
    stack:      YjrStack,
    hash:       Vec< YjrHash>,
    string:     Vec< String>,
    binary:     Vec< UserBinary >,
}

impl YjrEnviroment {
    fn insert_user_word(&mut self, name: &str, word: UserWord) {
        self.user_words.insert(name.to_string(), word);
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
            let mut new_code: WordCode = if token == "#def" {
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
                    if let WordCode::Number(ln) = w[0] {
                        if ln.fract() != 0.0 {
                            panic!("Loop count must be a integer");
                        }
                        for _ in 0.. (ln as usize) {
                            for i in 1..w.len() {
                                main_code.push( w[i].clone() );
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
                    if let WordCode::Symbol(ref s) = w[0] {
                        if s.starts_with("$") {
                            panic!("Word's name can't begin with $");
                        }
                        let mut new_word : UserWord = Vec::new();
                        for i in 1..w.len() {
                            new_word.push( w[i].clone() );
                        }
                        self.insert_user_word(s, new_word);
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
                    WordCode::Number( ln as TNT )
                } else {
                    panic!("']' list macro appears without begin '['");
                }
            } else {
                // first item of word is name.
                if let Some(ref mut w) = word_code {
                    if w.len() == 0 {
                        if check_symbol(&token) {
                            w.push( WordCode::Symbol(token.to_string()) );
                            continue;
                        } else {
                            panic!("Word name must be a alphabetnumberic");
                        }
                    }
                }

                // do some translate in second pass
                if let Some(n) = check_number( token) {
                    WordCode::Number(n)
                } else {
                    WordCode::Symbol(token.to_string())
                }
            };

            // second pass: translate symbol to native or user word.
            let mut push_byte = |x: WordCode| {
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

            let mut symbol = match &new_code {
                WordCode::Number(_) => {
                    push_byte(new_code.clone());
                    continue;
                },
                WordCode::Symbol(s) => {
                    s.clone()
                },
                _ => {
                    panic!("new_code must a symbol or number after first pass")
                }
            };

            // checking is a keyword
            if symbol == "true" {
                new_code = WordCode::Number(1.0);
                push_byte(new_code);
                continue;
            }
            if symbol == "false" {
                new_code = WordCode::Number(0.0);
                push_byte(new_code);
                continue;
            }
            if symbol == "null" {
                new_code = WordCode::Symbol("".to_string());
                push_byte(new_code);
                continue;
            }

            // checking is a native word
            if self.native_words.get(&symbol).is_some() {
                new_code = WordCode::Native(symbol.clone());
                push_byte(new_code);
                continue;
            }

            // checking is a user word
            if self.user_words.get(&symbol).is_some() {
                new_code = WordCode::User(symbol.clone());
                push_byte(new_code);
                continue;
            }

            // checking is a valid symbol
            if symbol.starts_with("$") {
                symbol.remove(0);
                if !check_symbol( &symbol ) {
                    panic!("Symbol must include alphanumbric or '_'");
                }
                push_byte(new_code);
                continue;
            }

            panic!("An symbol can't bind to user/native word!");
        }

        if !word_code.is_none() || !loop_code.is_none() || !list_count.is_none() {
            panic!("#loop list or word without ending");
        }

        main_code
    }

    pub fn new() -> Self {
        let mut ret = YjrEnviroment {
            user_words: HashMap::new(),
            native_words: HashMap::new(),
        };
        builtin::load_builtin(&mut ret);
        ret
    }

    pub fn insert_native_word(&mut self, name: &str, word: fn() -> Box<dyn NativeWord>) {
        self.native_words.insert(name.to_string(), word);
    }

    pub fn build(&mut self, code: &str ) -> YjrRuntime {
        let binary = self.compile(code);
        todo!()
    }

    pub fn run(&mut self, runtime: &mut YjrRuntime) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::YjrEnviroment;

    #[test]
    fn simple_test() {
    }
}

