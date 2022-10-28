use crate::TNT;
use crate::vector::Vector;
use crate::runtime::{YjrEnviroment, NativeWord, YjrStack, SharedVector};

use crate::faust::faust_help::{FaustDsp, ParamIndex};
use crate::faust::auto::*;

pub struct OscWord {
    ov: Option<SharedVector>,
    dsp: OsOsc::dsp,
}
impl OscWord {
    pub fn new(env: &YjrEnviroment) -> Box<dyn NativeWord> {
        let mut dsp = OsOsc::dsp::new();
        dsp.init( env.query("SampleRate").0 );
        Box::new( OscWord{
            ov: None,
            dsp: dsp
        })
    }
}

impl NativeWord for OscWord {
    fn run(&mut self, stack: &mut YjrStack) {
        let freq = stack.pop_number();
        self.dsp.set_param( ParamIndex(0), freq);
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

