use crate::TNT;
use crate::vector::Vector;
use crate::stack::{YjrStack, YjrHash, SharedVector};
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
                if stack.top().is_vector() {
                    let a = stack.pop_vector();
                    let b = SharedVector::new( a.vec().$op() );
                    stack.push_vector(b);
                } else {
                    let a = stack.pop_number();
                    let b = a.$op();
                    stack.push_number(b);
                }
            }
        }
    }
}

math_vector_unary_op!(Abs, abs);
math_vector_unary_op!(Acos, acos);
math_vector_unary_op!(Acosh, acosh);
math_vector_unary_op!(Asin, asin);
math_vector_unary_op!(Asinh, asinh);
math_vector_unary_op!(Atan, atan);
math_vector_unary_op!(Atanh, atanh);
math_vector_unary_op!(Cbrt, cbrt);
math_vector_unary_op!(Ceil, ceil);
math_vector_unary_op!(Cos, cos);
math_vector_unary_op!(Cosh, cosh);
math_vector_unary_op!(Exp, exp);
math_vector_unary_op!(Exp2, exp2);
math_vector_unary_op!(Floor, floor);
math_vector_unary_op!(Fract, fract);
math_vector_unary_op!(Ln, ln);
math_vector_unary_op!(Log10, log10);
math_vector_unary_op!(Recip, recip);
math_vector_unary_op!(Round, round);
math_vector_unary_op!(Sin, sin);
math_vector_unary_op!(Sinh, sinh);
math_vector_unary_op!(Sqrt, sqrt);
math_vector_unary_op!(Tan, tan);
math_vector_unary_op!(Tanh, tanh);
math_vector_unary_op!(Trunc, trunc);

pub fn insert_native_words(env: &mut YjrEnviroment) {
    // vector to number
    env.insert_native_word("sum",  Sum::new);
    env.insert_native_word("mean",  Mean::new);
    env.insert_native_word("var",  Var::new);

    // vector and vector to number
    env.insert_native_word("dot",  Dot::new);

    // vector to vector , number to number
    env.insert_native_word("abs",  Abs::new);
    env.insert_native_word("acos",  Acos::new);
    env.insert_native_word("acosh",  Acosh::new);
    env.insert_native_word("asin",  Asin::new);
    env.insert_native_word("asinh",  Asinh::new);
    env.insert_native_word("atan",  Atan::new);
    env.insert_native_word("atanh",  Atanh::new);
    env.insert_native_word("cbrt",  Cbrt::new);
    env.insert_native_word("ceil",  Ceil::new);
    env.insert_native_word("cos",  Cos::new);
    env.insert_native_word("cosh",  Cosh::new);
    env.insert_native_word("exp",  Exp::new);
    env.insert_native_word("exp2",  Exp2::new);
    env.insert_native_word("floor",  Floor::new);
    env.insert_native_word("fract",  Fract::new);
    env.insert_native_word("ln",  Ln::new);
    env.insert_native_word("log10",  Log10::new);
    env.insert_native_word("recip",  Recip::new);
    env.insert_native_word("round",  Round::new);
    env.insert_native_word("sin",  Sin::new);
    env.insert_native_word("sinh",  Sinh::new);
    env.insert_native_word("sqrt",  Sqrt::new);
    env.insert_native_word("tan",  Tan::new);
    env.insert_native_word("tanh",  Tanh::new);
    env.insert_native_word("trunc",  Trunc::new);
}

