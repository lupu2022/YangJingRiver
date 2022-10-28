use crate::TNT;
use crate::vector::Vector;
use crate::runtime::{NativeWord, YjrStack, SharedVector};

use crate::faust::faust_help::FaustDsp;
use crate::faust::auto::*;

pub struct OscWord {
    ov: Option<SharedVector>,
    dsp: OsOsc::dsp,
}
impl OscWord {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new( OscWord{
            ov: None,
            dsp: OsOsc::dsp::new()
        })
    }
}

impl NativeWord for OscWord {
    fn run(&mut self, stack: &mut YjrStack) {
        let count = stack.pop_number() as usize;
        if self.ov == None {
            self.ov = Some( SharedVector::new( Vector::<TNT>::zeros(count) ) );
        }
        if let Some(v) = &self.ov {
            let mut d = v.vec_mut();
            let mut output = vec![ d.mut_data() ];
            self.dsp.compute(count as i32, &vec![], &mut output);

            stack.push_vector(v.clone());
        }
        return;
    }
}

