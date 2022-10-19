use crate::TNT;
use crate::stack::{YjrStack, YjrHash, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};


struct Add {}
impl Add {
    pub fn new()->Box<dyn NativeWord> {
        Box::new(Add {})
    }
}

impl NativeWord for Add {
    fn boot(&mut self, stack: &mut YjrStack, local_: &mut YjrHash, global_: &mut YjrHash) {
        self.tick(stack);
    }

    fn tick(&mut self, stack: &mut YjrStack) {
        if stack.top().is_vector() {
            let a = stack.pop_vector();
            if stack.top().is_vector() {
                let b = stack.pop_vector();
                let c = &*a.vec() + &*b.vec();
                stack.push_vector( SharedVector::new(c) );
                return;
            }
            let b = stack.pop_number();
            let c = &*a.vec() + &b;
            stack.push_vector( SharedVector::new(c) );
            return;
        }
        let a = stack.pop_number();
        let b = stack.pop_number();
        stack.push_number(a + b);
    }
}


pub fn load_builtin(env: &mut YjrEnviroment) {
    env.insert_native_word("+",     Add::new);

    /*
    env.insert_native_word("-",     sub);
    env.insert_native_word("*",     mul);
    env.insert_native_word("/",     div);
    */
}
