//use crate::TNT;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

struct Get {}
impl Get {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for Get {
    fn run(&mut self, stack: &mut YjrStack, hash: &mut YjrHash) {
        let name = stack.pop_string();
        let item: YjrItem = hash.get(&name);
        stack.push(item);
    }
}

struct Set {}
impl Set {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for Set {
    fn run(&mut self, stack: &mut YjrStack, hash: &mut YjrHash) {
        let name = stack.pop_string();
        let item: YjrItem = stack.pop();
        hash.set(&name, item);
    }
}

macro_rules! base_stack_op {
    ($name:ident, $op:ident) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
                stack.$op();
            }
        }
    }
}

base_stack_op!{DropW , drop}
base_stack_op!{Dup , dup}
base_stack_op!{Dup2 , dup2}
base_stack_op!{Swap , swap}
base_stack_op!{Rot , rot}

macro_rules! base_binary_op {
    ($name:ident, $op:tt) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
                if stack.top().is_vector() {
                    let a = stack.pop_vector();
                    let b = stack.pop_vector();
                    let c = &*b.vec() $op &*a.vec();
                    stack.push_vector( SharedVector::new(c) );
                    return;
                }
                let a = stack.pop_number();
                if stack.top().is_vector() {
                    let b = stack.pop_vector();
                    let c = &*b.vec() $op a;
                    stack.push_vector( SharedVector::new(c) );
                    return;
                }
                let b = stack.pop_number();
                let c = b $op a;
                stack.push_number(c);
            }
        }
    }
}


base_binary_op!{Add , +}
base_binary_op!{Sub , -}
base_binary_op!{Mod , %}
base_binary_op!{Mul , *}
base_binary_op!{Div , /}

pub fn insert_native_words(env: &mut YjrEnviroment) {
    // Stack Operator
    env.insert_native_word("drop",  DropW::new);
    env.insert_native_word("dup",  Dup::new);
    env.insert_native_word("dup2",  Dup2::new);
    env.insert_native_word("swap",  Swap::new);
    env.insert_native_word("rot",  Rot::new);

    // Hash Operator
    env.insert_native_word("@",  Get::new);
    env.insert_native_word("!",  Set::new);

    // basic Arithmetic
    env.insert_native_word("+",  Add::new);
    env.insert_native_word("-",  Sub::new);
    env.insert_native_word("%",  Mod::new);
    env.insert_native_word("*",  Mul::new);
    env.insert_native_word("/",  Div::new);
}

