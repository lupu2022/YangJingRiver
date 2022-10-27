use crate::TNT;
use crate::vector::Vector;
use crate::stack::{YjrStack, YjrHash, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

use crate::faust::faust_help::FaustDsp;
use crate::faust::auto::*;


struct NoiseWord {
    dsp: NoNoise::dsp,
}
impl NoiseWord {
    pub fn new() -> Box<dyn NativeWord> {
        Box::new( NoiseWord{
            dsp: NoNoise::dsp::new()
        })
    }
}
impl NativeWord for NoiseWord {
    fn run(&mut self, stack: &mut YjrStack, _hash: &mut YjrHash) {
    }
}

