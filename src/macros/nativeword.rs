#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! math_vector_number_op {
    ($name:ident, $op:ident) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
                let a = stack.pop_vector();
                let b = a.vec().$op();
                stack.push_number(b);
            }
        }
    }
}

#[macro_export]
macro_rules! math_vector_unary_op {
    ($name:ident, $op:ident) => {
        struct $name {}
        impl $name {
            pub fn new()->Box<dyn NativeWord> {
                Box::new($name {})
            }
        }
        impl NativeWord for $name {
            fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
                let a = stack.pop_vector();
                stack.push_number(b);
            }
        }
    }
}

