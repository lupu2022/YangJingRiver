// auto generated files, don't edit it.

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use crate::faust::faust_help::*;


pub struct dsp {
	iRec0: [i32;2],
	fSampleRate: i32,
}

impl FaustDsp for dsp {
	type T = F32;

	fn new() -> dsp {
		dsp {
			iRec0: [0;2],
			fSampleRate: 0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) {
		m.declare("filename", "no.noise.dsp");
		m.declare("name", "no.noise");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.4");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}

	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iRec0[(l0) as usize] = 0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		dsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}

	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}

	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("no.noise");
		ui_interface.close_box();
	}

	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			_ => None,
		}
	}

	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			_ => {}
		}
	}

	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec0[1]), 12345);
			*output0 = 4.656613e-10 * ((self.iRec0[0]) as F32);
			self.iRec0[1] = self.iRec0[0];
		}
	}

}

