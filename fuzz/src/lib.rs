#![feature(portable_simd)]
mod clipper;
mod filter;
mod params;
mod shared {
  pub mod float_ext;
}
pub use params::Params;
use {clipper::Clipper, filter::Filter, params::Smoother};

pub struct Fuzz {
  pre_filter: Filter,
  clipper: Clipper,
  tone: Filter,
}

impl Fuzz {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pre_filter: Filter::new(sample_rate),
      clipper: Clipper::new(),
      tone: Filter::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let pre_filter = params.pre_filter.next();
    let gain = params.gain.next();
    let bias = params.bias.next();
    let tone = params.tone.next();
    let volume = params.volume.next();

    let pre_filter_out = self.pre_filter.process(input, pre_filter);
    let clipper_out = self.clip(pre_filter_out, gain, bias);
    let tone_out = self.tone.process(clipper_out, tone);
    tone_out * volume
  }

  fn clip(&mut self, input: f32, gain: f32, bias: f32) -> f32 {
    let scaled_input = input * gain;
    let clipper_input = scaled_input + scaled_input.abs() * bias;
    let gain_compensation = 1. - (bias * 0.5);
    self.clipper.process(clipper_input * gain_compensation)
  }
}
