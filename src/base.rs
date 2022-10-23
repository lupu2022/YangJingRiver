use crate::TNT;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};
use crate::vector::Vector;

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

struct GetWith {}
impl GetWith {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for GetWith {
    fn run(&mut self, stack: &mut YjrStack, hash: &mut YjrHash) {
        let name = stack.pop_string();
        let default = stack.pop();
        if hash.find(&name) {
            let item: YjrItem = hash.get(&name);
            stack.push(item);
        } else {
            stack.push(default);
        }
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

macro_rules! vector_creator {
    ($name:ident, $op:ident) => {
        struct $name {
            data: Option<SharedVector>
        }
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {
                    data: None
                })
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
                if let Some(ref v) = self.data {
                    stack.pop_number();
                    stack.push_vector( v.clone() );
                    return;
                }

                let size = stack.pop_number();
                if size.fract() != 0.0 {
                    panic!("Create vector with size must be a integer!");
                }
                let size = size as usize;
                let v = SharedVector::new( Vector::<TNT>::$op(size) );
                self.data = Some(v.clone());
                stack.push_vector(v);
            }
        }
    }
}

vector_creator!{Zeros, zeros}
vector_creator!{Ones, ones}

pub fn insert_native_words(env: &mut YjrEnviroment) {
    // Stack Operator
    env.insert_native_word("drop",  DropW::new);
    env.insert_native_word("dup",  Dup::new);
    env.insert_native_word("dup2",  Dup2::new);
    env.insert_native_word("swap",  Swap::new);
    env.insert_native_word("rot",  Rot::new);

    // Hash Operator
    env.insert_native_word("@",  Get::new);
    env.insert_native_word("@~", GetWith::new);
    env.insert_native_word("!",  Set::new);

    // creator of vector
    env.insert_native_word("zeros~", Zeros::new);
    env.insert_native_word("ones~",  Ones::new);
}

