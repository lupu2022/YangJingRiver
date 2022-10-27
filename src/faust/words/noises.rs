use crate::TNT;
use crate::vector::Vector;
use crate::stack::{YjrStack, YjrHash, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

use crate::faust::faust_help::FaustDsp;
use crate::faust::auto::*;

pub struct NoiseWord {
    ov: Option<SharedVector>,
    dsp: NoNoise::dsp,
}
impl NoiseWord {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new( NoiseWord{
            ov: None,
            dsp: NoNoise::dsp::new()
        })
    }
}

impl NativeWord for NoiseWord {
    fn run(&mut self, stack: &mut YjrStack) {
        let size = stack.pop_number() as usize;
        if self.ov == None {
            self.ov = Some( SharedVector::new( Vector::<TNT>::zeros(size) ) );
        }
        if let Some(v) = &self.ov {
            let mut d = v.vec_mut();
            let mut output = vec![ d.mut_data() ];
            self.dsp.compute(size as i32, &vec![], &mut output);

            stack.push_vector(v.clone());
        }
        return;
    }
}

