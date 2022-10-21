use std::collections::HashMap;

use crate::TNT;
use crate::stack::{YjrStack, YjrHash};
use crate::base;
use crate::math;

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
    fn run(&mut self, stack: &mut YjrStack, hash: &mut YjrHash);
}

pub struct YjrEnviroment {
    user_words: HashMap<String, UserWord >,
    native_words: HashMap<String, fn()->Box<dyn NativeWord> >
}

impl YjrEnviroment {
    fn insert_user_word(&mut self, name: &str, word: UserWord) {
        self.user_words.insert(name.to_string(), word);
    }

    fn compile(&mut self, txt: &str) -> UserWord {
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

        fn remove_comment(txt: &str) -> String {
            let mut contents = "".to_string();
            for line in txt.lines() {
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

        let tokens = tokenize( &remove_comment(txt) );

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
                        panic!("First item of a word must be a word name!");
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
            let mut push_code = |x: WordCode| {
                if let Some(ref mut uw) = loop_code {
                    uw.push(x);
                } else if let Some(ref mut uw) = word_code {
                    uw.push(x);
                } else {
                    main_code.push(x);
                }

                if let Some(n) = list_count {
                    list_count = Some(n+1);
                }
            };

            let mut symbol = match &new_code {
                WordCode::Number(_) => {
                    push_code(new_code.clone());
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
                push_code(new_code);
                continue;
            }
            if symbol == "false" {
                new_code = WordCode::Number(0.0);
                push_code(new_code);
                continue;
            }
            if symbol == "null" {
                new_code = WordCode::Symbol("".to_string());
                push_code(new_code);
                continue;
            }

            // checking is a native word
            if self.native_words.get(&symbol).is_some() {
                new_code = WordCode::Native(symbol.clone());
                push_code(new_code);
                continue;
            }

            // checking is a user word
            if self.user_words.get(&symbol).is_some() {
                new_code = WordCode::User(symbol.clone());
                push_code(new_code);
                continue;
            }

            // checking is a valid symbol
            if symbol.starts_with("$") {
                symbol.remove(0);
                if !check_symbol( &symbol ) {
                    panic!("Symbol must include alphanumbric or '_'");
                }
                push_code(new_code);
                continue;
            }

            panic!("An symbol can't bind to user/native word!");
        }

        if !word_code.is_none() || !loop_code.is_none() || !list_count.is_none() {
            panic!("#loop list or word without ending");
        }

        main_code
    }

    fn create_native(&self, name: &str) -> Box<dyn NativeWord> {
        let ret = self.native_words.get(name);
        if let Some(f) = ret {
            return f();
        }
        panic!("Can't find native word by name")
    }

    fn get_user(&self, name: &str) -> &UserWord {
        let ret = self.user_words.get(name);
        if let Some(w) = ret {
            return w
        }
        panic!("Can't find native word by name")
    }

    pub fn new() -> Self {
        let mut ret = YjrEnviroment {
            user_words: HashMap::new(),
            native_words: HashMap::new(),
        };
        base::insert_native_words(&mut ret);
        math::insert_native_words(&mut ret);
        ret
    }

    pub fn insert_native_word(&mut self, name: &str, word: fn() -> Box<dyn NativeWord>) {
        self.native_words.insert(name.to_string(), word);
    }

    pub fn build(&mut self, txt: &str ) -> YjrRuntime {
        let main_code = self.compile(txt);
        YjrRuntime::new(self, &main_code)
    }
}


pub struct YjrRuntime {
    pub stack:   YjrStack,
    pub hash:    YjrHash,
    strings:     Vec< String>,
    binarys:     Vec< UserBinary >,
    natives:     Vec< Box<dyn NativeWord> >,
}

impl YjrRuntime {
    fn string_id(&mut self, s: &str) -> usize {
        for i in 0..self.strings.len() {
            if s == &self.strings[i] {
                return i;
            }
        }
        let ret = self.strings.len();
        self.strings.push(s.to_string());
        ret
    }

    fn linking(&mut self, env: &YjrEnviroment, main_code: &UserWord) {
        let id:usize = self.binarys.len();
        self.binarys.push( Vec::new());
        self.hash.inc();

        let mut bin = Vec::new();
        for code in main_code {
            match code {
                WordCode::Number(n) => {
                    bin.push( WordByte::Number(*n) );
                },
                WordCode::Symbol(s) => {
                    bin.push( WordByte::Symbol(self.string_id(s)) );
                },
                WordCode::Native(s) => {
                    bin.push( WordByte::Native( self.natives.len() ) );
                    self.natives.push( env.create_native(s) );
                },
                WordCode::User(s) => {
                    bin.push( WordByte::User( self.binarys.len() ) );
                    let uw = env.get_user(s);
                    self.linking(env, uw);
                },
            }
        }

        self.binarys[id] = bin;
    }

    fn new(env: &YjrEnviroment, main_code: &UserWord) -> Self {
        let mut rt = YjrRuntime {
            stack: YjrStack::new(),
            hash:  YjrHash::new(),
            strings: Vec::new(),
            binarys: Vec::new(),
            natives: Vec::new(),
        };

        rt.linking(env, main_code);
        rt
    }

    fn run_(&mut self, i: usize) {
        self.hash.moveto(i);
        for j in 0..self.binarys[i].len() {
            let w = self.binarys[i][j].clone();
            match w {
                WordByte::Number(n) => {
                    self.stack.push_number(n);
                },
                WordByte::Symbol(s) => {
                    self.stack.push_string( self.strings[s].to_string() );
                },
                WordByte::Native(n) => {
                    self.natives[n].run(&mut self.stack, &mut self.hash);
                },
                WordByte::User(w) => {
                    assert!(w == (i + 1));
                    self.run_(i+1);
                },
            }
        }
    }
    pub fn run(&mut self) {
        self.run_(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::YjrEnviroment;

    #[test]
    fn simple_test() {
    }
}

