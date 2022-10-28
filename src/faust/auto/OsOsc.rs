// auto generated files, don't edit it.

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use crate::faust::faust_help::*;



pub struct dspSIG0 {
	iVec0: [i32;2],
	iRec0: [i32;2],
}

impl dspSIG0 {

	fn get_num_inputsdspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsdspSIG0(&self) -> i32 {
		return 1;
	}

	fn instance_initdspSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec0[(l1) as usize] = 0;
		}
	}

	fn filldspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec0[0] = 1;
			self.iRec0[0] = (i32::wrapping_add(self.iVec0[1], self.iRec0[1])) % 65536;
			table[(i1) as usize] = F32::sin(9.58738e-05 * ((self.iRec0[0]) as F32));
			self.iVec0[1] = self.iVec0[0];
			self.iRec0[1] = self.iRec0[0];
		}
	}

}


pub fn newdspSIG0() -> dspSIG0 {
	dspSIG0 {
		iVec0: [0;2],
		iRec0: [0;2],
	}
}
static mut ftbl0dspSIG0: [F32;65536] = [0.0;65536];
pub struct dsp {
	fHslider0: F32,
	fSampleRate: i32,
	fConst0: F32,
	fRec1: [F32;2],
}

impl FaustDsp for dsp {
	type T = F32;

	fn new() -> dsp {
		dsp {
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fRec1: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) {
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.8");
		m.declare("filename", "os.osc.dsp");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.5");
		m.declare("name", "os.osc");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.3");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.2");
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
		let mut sig0: dspSIG0 = newdspSIG0();
		sig0.instance_initdspSIG0(sample_rate);
		sig0.filldspSIG0(65536, unsafe { &mut ftbl0dspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 4.4e+02;
	}
	fn instance_clear(&mut self) {
		for l2 in 0..2 {
			self.fRec1[(l2) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = 1.0 / F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
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
		ui_interface.open_vertical_box("os.osc");
		ui_interface.add_horizontal_slider("freq", ParamIndex(0), 4.4e+02, 25.0, 1.1e+04, 0.1);
		ui_interface.close_box();
	}

	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fHslider0),
			_ => None,
		}
	}

	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fHslider0 = value }
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
		let mut fSlow0: F32 = self.fConst0 * self.fHslider0;
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.fRec1[0] = fSlow0 + (self.fRec1[1] - F32::floor(fSlow0 + self.fRec1[1]));
			*output0 = unsafe { ftbl0dspSIG0[(((65536.0 * self.fRec1[0]) as i32)) as usize] };
			self.fRec1[1] = self.fRec1[0];
		}
	}

}

