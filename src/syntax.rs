use std::collections::HashMap;

use crate::TNT;
use crate::stack::{YjrStack, YjrHash};

#[derive(Debug, PartialEq, Clone)]
enum SyntaxItem {
    Number(TNT),
    Str(String),
    Symbol(String),
}

#[derive(Debug, PartialEq, Clone)]
enum WordCode {
    Number(TNT),
    Str(usize),
    Native(usize),
    User(usize),
}
pub type UserWord = Vec<WordCode>;

pub struct YjrEnviroment {
    user_words : Vec<UserWord>,
    native_words : Vec< fn(&mut YjrStack) >,
}

pub struct YjrRuntime {
    stack:      YjrStack,
    hash:       Vec<YjrHash>,
    binary:     UserWord,
}

impl YjrEnviroment {
    pub fn new() -> Self {
        YjrEnviroment {
            user_words:     Vec::new(),
            native_words:   Vec::new(),
        }
    }

    pub fn compile(&mut self, code: &str) -> UserWord {
        fn tokenize(expr: &str) -> Vec<String> {
            expr.replace("\n", " ")
                .replace("{", " ")
                .replace("}", " ")
                .replace("(", " ")
                .replace(")", " ")
                .split_whitespace()
                .map(|x| x.to_string())
                .collect()
        }

        let tokens = tokenize(code);


        todo!()
    }

    pub fn build(&self, uw: &UserWord) -> YjrRuntime {
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



