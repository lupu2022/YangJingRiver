use crate::TNT;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};
use crate::vector::Vector;

macro_rules! base_stack_op {
    ($name:ident, $op:ident) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack) {
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
            fn run(&mut self, stack: &mut YjrStack) {
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

// Three basic operators
struct Get {}
impl Get {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for Get {
    fn name(&self) -> &'static str {
        return "get";
    }
    fn run(&mut self, _stack: &mut YjrStack) {
    }
}

struct GetWith {}
impl GetWith {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for GetWith {
    fn name(&self) -> &'static str {
        return "get~";
    }
    fn run(&mut self, _stack: &mut YjrStack) {
    }
}

struct Set {}
impl Set {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{})
    }
}
impl NativeWord for Set {
    fn name(&self) -> &'static str {
        return "set";
    }
    fn run(&mut self, _stack: &mut YjrStack) {
    }
}

pub fn insert_native_words(env: &mut YjrEnviroment) {
    // Stack Operator
    env.insert_native_word("drop",  DropW::new);
    env.insert_native_word("dup",  Dup::new);
    env.insert_native_word("dup2",  Dup2::new);
    env.insert_native_word("swap",  Swap::new);
    env.insert_native_word("rot",  Rot::new);

    // Only three hash operators
    env.insert_native_word("@",  Get::new);
    env.insert_native_word("@~", GetWith::new);
    env.insert_native_word("!",  Set::new);

    // creator of vector
    env.insert_native_word("zeros~", Zeros::new);
    env.insert_native_word("ones~",  Ones::new);
}

