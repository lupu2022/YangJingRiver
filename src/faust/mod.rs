use crate::runtime::{YjrEnviroment};

mod faust_help;
mod auto;
mod words;

pub fn insert_native_words(env: &mut YjrEnviroment) {
    env.insert_native_word("dsp.no.noise", words::noises::NoiseWord::new);
    env.insert_native_word("dsp.os.osc", words::oscillators::OscWord::new);
}

