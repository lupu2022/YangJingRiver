#[macro_export]
macro_rules! builtin_binary_op {
    ($name:ident, $op:tt) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn boot(&mut self, stack: &mut YjrStack, local_: &mut YjrHash, global_: &mut YjrHash) {
                self.tick(stack);
            }

            fn tick(&mut self, stack: &mut YjrStack) {
                if stack.top().is_vector() {
                    let a = stack.pop_vector();
                    if stack.top().is_vector() {
                        let b = stack.pop_vector();
                        let c = &*a.vec() $op &*b.vec();
                        stack.push_vector( SharedVector::new(c) );
                        return;
                    }
                    let b = stack.pop_number();
                    let c = &*a.vec() $op &b;
                    stack.push_vector( SharedVector::new(c) );
                    return;
                }
                let a = stack.pop_number();
                let b = stack.pop_number();
                stack.push_number(a $op b);
            }
        }
    }
}

