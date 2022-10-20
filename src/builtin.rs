use crate::TNT;
use crate::stack::{YjrStack, YjrHash, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

builtin_binary_op!{Add , +}
builtin_binary_op!{Sub , -}
builtin_binary_op!{Mod , %}
builtin_binary_op!{Mul , *}
builtin_binary_op!{Div , /}

pub fn load_builtin(env: &mut YjrEnviroment) {
    env.insert_native_word("+",  Add::new);
    env.insert_native_word("-",  Sub::new);
    env.insert_native_word("%",  Mod::new);
    env.insert_native_word("*",  Mul::new);
    env.insert_native_word("/",  Div::new);
}
