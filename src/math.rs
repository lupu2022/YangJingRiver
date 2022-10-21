use crate::TNT;
use crate::vector::Vector;
use crate::stack::{YjrStack, YjrHash, YjrItem, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

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
// vector's sum, mean, variance
math_vector_number_op!(Sum,  sum);
math_vector_number_op!(Mean, mean);
math_vector_number_op!(Var,  variance);

// vector dot
struct Dot {}
impl Dot {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new(Dot{})
    }
}
impl NativeWord for Dot {
    fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
        let a: SharedVector = stack.pop_vector();
        let b: SharedVector = stack.pop_vector();

        let c = a.vec().dot(&b.vec());
        stack.push_number(c);
    }
}

struct Ones {
    d:  Option<SharedVector>,
}
impl Ones {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new( Ones{
            d: None,
        })
    }
}
impl NativeWord for Ones {
    fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
        let size: usize = stack.pop_number() as usize;
        if self.d.is_none() {
            self.d = Some( SharedVector::new( Vector::<TNT>::ones(size) ) );
        }

        stack.push_vector( self.d.as_ref().unwrap().clone() );
    }
}

struct Zeros {
    d:  Option<SharedVector>,
}
impl Zeros {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new( Zeros{
            d: None,
        })
    }
}
impl NativeWord for Zeros {
    fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
        let size: usize = stack.pop_number() as usize;
        if self.d.is_none() {
            self.d = Some( SharedVector::new( Vector::<TNT>::zeros(size) ) );
        }

        stack.push_vector( self.d.as_ref().unwrap().clone() );
    }
}

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


pub fn insert_native_words(env: &mut YjrEnviroment) {
    env.insert_native_word("ones",  Ones::new);
    env.insert_native_word("zeros",  Zeros::new);

    env.insert_native_word("sum",  Sum::new);
    env.insert_native_word("mean",  Mean::new);
    env.insert_native_word("var",  Var::new);

    env.insert_native_word("dot",  Dot::new);
}

