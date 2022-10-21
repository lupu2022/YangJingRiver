//use crate::TNT;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

builtin_binary_op!{Add , +}
builtin_binary_op!{Sub , -}
builtin_binary_op!{Mod , %}
builtin_binary_op!{Mul , *}
builtin_binary_op!{Div , /}

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

pub fn load_builtin(env: &mut YjrEnviroment) {
    // Hash Operator
    env.insert_native_word("@",  Get::new);

    // basic Arithmetic
    env.insert_native_word("+",  Add::new);
    env.insert_native_word("-",  Sub::new);
    env.insert_native_word("%",  Mod::new);
    env.insert_native_word("*",  Mul::new);
    env.insert_native_word("/",  Div::new);
}
