use crate::TNT;
use crate::vector::Vector;
use crate::stack::{YjrStack, YjrHash, SharedVector};
use crate::runtime::{YjrEnviroment, NativeWord};

use crate::faust::auto::*;

struct NoiseWord {
    dsp: NoNoise::dsp,
}


