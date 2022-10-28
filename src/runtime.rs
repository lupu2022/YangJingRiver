use std::cell::{RefCell, RefMut, Ref};
use std::collections::HashMap;
use std::rc::Rc;

use crate::vector::Vector;
use crate::TNT;
use crate::base;
use crate::math;
use crate::faust;

#[derive(Debug, PartialEq, Clone)]
pub struct SharedVector(Rc<RefCell<Vector<TNT>>>);

impl SharedVector {
    pub fn new(v: Vector<TNT>) -> Self {
        SharedVector(Rc::new( RefCell::new(v) ))
    }

    pub fn vec(&self) -> Ref<'_, Vector<TNT>> {
        self.0.borrow()
    }

    pub fn vec_mut(&self) -> RefMut<'_, Vector<TNT>> {
        self.0.borrow_mut()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum YjrItem {
    S( String ),        // string, used as flag or hash key
    N( TNT ),           // number
    V( SharedVector ),  // vector
}


impl YjrItem {
    pub fn new() -> Self {
        YjrItem::S("".to_string())
    }

    pub fn is_string(&self) -> bool {
        match self {
            YjrItem::S(_) => true,
            _ => false,
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            YjrItem::N(_) => true,
            _ => false,
        }
    }
    pub fn is_vector(&self) -> bool {
        match self {
            YjrItem::V(_) => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            YjrItem::S(v) => {
                if v == "" {
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    // consuming
    pub fn as_string(self) -> String {
        match self {
            YjrItem::S(s) => s,
            _ => panic!("Item is not string")
        }
    }

    pub fn as_number(self) -> TNT {
        match self {
            YjrItem::N(n) => n,
            _ => panic!("Item is not number")
        }
    }

    pub fn as_vector(self) -> SharedVector {
        match self {
            YjrItem::V(v) => v,
            _ => panic!("Item is not vector")
        }
    }
}

#[derive(Debug)]
pub struct YjrStack {
    data:   Vec<YjrItem>
}

impl YjrStack {
    pub fn new() -> Self {
        YjrStack {
            data: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn top(&self) -> &YjrItem {
        self.data.last().unwrap()
    }

    pub fn pop(&mut self) -> YjrItem {
        self.data.pop().unwrap()
    }

    pub fn drop(&mut self) {
        self.data.pop();
    }

    pub fn dup(&mut self) {
        let new_item = self.top().clone();
        self.data.push(new_item);
    }

    pub fn dup2(&mut self) {
        let top2 = self.data[ self.data.len() - 2].clone();
        let top1 = self.data[ self.data.len() - 1].clone();

        self.data.push(top2);
        self.data.push(top1);
    }

    pub fn swap(&mut self) {
        let top1 = self.data.pop().unwrap();
        let top2 = self.data.pop().unwrap();

        self.data.push(top1);
        self.data.push(top2);
    }

    pub fn rot(&mut self) {
        let top1 = self.data.pop().unwrap();
        let top2 = self.data.pop().unwrap();
        let top3 = self.data.pop().unwrap();

        self.data.push(top2);
        self.data.push(top1);
        self.data.push(top3);
    }

    pub fn push(&mut self, item: YjrItem) {
        self.data.push( item );
    }

    fn push_string<T: ToString>(&mut self, s: T) {
        let item = YjrItem::S(s.to_string());
        self.data.push( item );
    }

    /*
    fn push_string_list(&mut self, sl: Vec<String>) {
        let lsize = sl.len();
        for s in sl {
            self.push_string(s);
        }
        self.push_number(lsize as TNT);
    }
    */

    pub fn push_number(&mut self, n: TNT) {
        let item = YjrItem::N(n);
        self.data.push(item);
    }

    pub fn push_number_list(&mut self, sl: Vec<TNT>) {
        let lsize = sl.len();
        for s in sl {
            self.push_number(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn push_vector(&mut self, v: SharedVector) {
        let item = YjrItem::V(v);
        self.data.push(item);
    }

    pub fn push_vector_list(&mut self, sl: Vec<SharedVector>) {
        let lsize = sl.len();
        for s in sl {
            self.push_vector(s);
        }
        self.push_number(lsize as TNT);
    }

    pub fn pop_string(&mut self) -> String {
        self.data.pop().unwrap().as_string()
    }

    pub fn pop_number(&mut self) -> TNT {
        self.data.pop().unwrap().as_number()
    }

    pub fn pop_vector(&mut self) -> SharedVector {
        self.data.pop().unwrap().as_vector()
    }

    pub fn pop_string_list(&mut self) -> Vec<String> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![String::new(); lsize];
        for i in 0..lsize {
            ret[lsize - i - 1] = self.pop_string();
        }
        return ret;
    }

    pub fn pop_number_list(&mut self) -> Vec<TNT> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![0.0; lsize];
        for i in 0..lsize {
            ret[lsize - i - 1] = self.pop_number();
        }
        return ret;
    }

    pub fn pop_vector_list(&mut self) -> Vec<SharedVector> {
        let lsize = self.pop_number() as usize;
        let mut ret = vec![];
        for _ in 0..lsize {
            ret.push( self.pop_vector() );
        }
        ret.reverse();
        ret
    }
}

pub struct YjrHash {
    maps:   Vec< HashMap<String, YjrItem>>,
    target: usize,
}

impl YjrHash {
    pub fn new() -> Self {
        YjrHash {
            maps:       Vec::new(),
            target:     0,
        }
    }
    pub fn inc(&mut self) {
        self.maps.push( HashMap::new() );
    }
    pub fn moveto(&mut self, i: usize) {
        if i < self.maps.len() {
            self.target = i;
        } else {
            panic!("Hash is out of range!");
        }
    }
    pub fn find(&self, _name: &str) -> bool {
        todo!()
    }
    pub fn get(&self, _name: &str) -> YjrItem {
        todo!()
    }
    pub fn set(&self, _name: &str, _item: YjrItem) {
        todo!()
    }
}



#[derive(Debug, PartialEq, Clone)]
enum WordCode {
    Number(TNT),
    Symbol(String),
    GetOperator(),
    SetOperator(),
    Native(String),
    User(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum WordByte {
    Number(TNT),
    Symbol(usize),
    GetOperator(),
    SetOperator(),
    Native(usize),
    User(usize),
}

type UserWord = Vec<WordCode>;
type UserBinary = Vec<WordByte>;
type EnvConfig =  (bool, i32, f32);
pub trait NativeWord  {
    fn run(&mut self, stack: &mut YjrStack);
}

pub struct YjrEnviroment {
    user_words: HashMap<String, UserWord >,
    native_words: HashMap<String, fn()->Box<dyn NativeWord> >,
    settings:   HashMap<String, EnvConfig>,
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
                    panic!("Can't define word in a loop macro.");
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
                    panic!("Can't ending a word or a loop in a list macro.");
                }

                if let Some(ref w) = loop_code {
                    // loop section ending
                    if w.len() == 0 {
                        panic!("#loop macro without loop count!");
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
                        panic!("First item must be a word name in #define");
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
            if symbol == "@" {
                new_code = WordCode::GetOperator();
                push_code(new_code);
                continue;
            }
            if symbol == "!" {
                new_code = WordCode::SetOperator();
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
            if symbol.starts_with("$") || symbol.starts_with("%") {
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

    pub fn new(r: i32) -> Self {
        let mut env = YjrEnviroment {
            user_words: HashMap::new(),
            native_words: HashMap::new(),
            settings: HashMap::new()
        };
        env.settings.insert("SampleRate".to_string() , (false, r, 0.0));

        base::insert_native_words(&mut env);
        math::insert_native_words(&mut env);
        faust::insert_native_words(&mut env);
        env
    }

    pub fn query(&self, _key: &str) -> EnvConfig {
        todo!()
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
                WordCode::GetOperator() => {
                    bin.push( WordByte::GetOperator());
                },
                WordCode::SetOperator() => {
                    bin.push( WordByte::SetOperator());
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
                WordByte::GetOperator() => {
                    let key = self.stack.pop_string();
                    let item: YjrItem = self.hash.get(&key);
                    self.stack.push(item);
                },
                WordByte::SetOperator() => {
                    let key = self.stack.pop_string();
                    let item: YjrItem = self.stack.pop();
                    self.hash.set(&key, item);
                },
                WordByte::Native(n) => {
                    self.natives[n].run(&mut self.stack);
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
    use crate::runtime::{YjrEnviroment,YjrStack,SharedVector};

    #[test]
    fn basic_stack() {
        let mut stack = YjrStack::new();

        stack.push_number(3.14);
        stack.push_string("Hello World");
        stack.push_string("Hello World".to_string());

        stack.push_vector( SharedVector::new(vector![1.0, 2.0, 3.0, 4.0, 5.0]) );

        stack.push_number(1949.0);
        stack.push_number(1979.0);
        stack.push_number(2.0);

        let l = stack.pop_number_list();
        let v = stack.pop_vector();
        let s = stack.pop_string();

        stack.push_number_list( vec![1.0, 2.0, 3.0] );

        println!("{:?}", stack);
        println!("{:?}", l);
        println!("{:?}", v.vec() );
        println!("{}", s);
    }

    #[test]
    fn simple_run() {
        let mut env = YjrEnviroment::new();
        let txt = "3.14 1.0 + floor ones~";
        let mut rt = env.build(txt);
        rt.run();
        println!("{:?}", rt.stack);
    }

    #[test]
    fn simple_faust() {
        let mut env = YjrEnviroment::new();
        let txt = "100 dsp.no.noise";
        let mut rt = env.build(txt);
        rt.run();
        println!("{:?}", rt.stack);

        rt.run();
        println!("{:?}", rt.stack);
    }
}

