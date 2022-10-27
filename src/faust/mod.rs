use crate::runtime::{YjrEnviroment, YjrRuntime};

mod faust_help;
mod auto;
mod words;

pub fn insert_native_words(env: &mut YjrEnviroment) {
    env.insert_native_word("dsp.no.noise", words::noises::NoiseWord::new);
}

