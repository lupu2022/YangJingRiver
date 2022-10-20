//use crate::TNT;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

builtin_binary_op!{Add , +}
builtin_binary_op!{Sub , -}
builtin_binary_op!{Mod , %}
builtin_binary_op!{Mul , *}
builtin_binary_op!{Div , /}

struct Get {
    item: YjrItem
}
impl Get {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Get{
            item: YjrItem::new(),
        })
    }
}
impl NativeWord for Get {
    fn boot(&mut self, stack: &mut YjrStack, hash: &mut YjrHash) {
    }
    fn tick(&mut self, stack: &mut YjrStack) {
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
