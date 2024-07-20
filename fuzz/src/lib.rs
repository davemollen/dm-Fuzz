#![feature(portable_simd)]
mod clipper;
mod filter;
mod smooth_parameters;
mod shared {
  pub mod float_ext;
}
use clipper::Clipper;
use filter::Filter;
use smooth_parameters::SmoothParameters;

pub struct Fuzz {
  smooth_parameters: SmoothParameters,
  pre_filter: Filter,
  clipper: Clipper,
  tone: Filter,
}

impl Fuzz {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_parameters: SmoothParameters::new(sample_rate),
      pre_filter: Filter::new(sample_rate),
      clipper: Clipper::new(),
      tone: Filter::new(sample_rate),
    }
  }

  pub fn map_filter_param(filter: f32) -> f32 {
    filter * filter * 0.175438596
  }

  pub fn initialize_params(
    &mut self,
    pre_filter: f32,
    gain: f32,
    bias: f32,
    tone: f32,
    volume: f32,
  ) {
    self
      .smooth_parameters
      .initialize(pre_filter, gain, bias, tone, volume);
  }

  pub fn process(
    &mut self,
    input: f32,
    pre_filter: f32,
    gain: f32,
    bias: f32,
    tone: f32,
    volume: f32,
  ) -> f32 {
    let (pre_filter, gain, bias, tone, volume) = self
      .smooth_parameters
      .process(pre_filter, gain, bias, tone, volume);

    let pre_filter_out = self.pre_filter.process(input, pre_filter);
    let clipper_out = self.clipper.process(pre_filter_out * gain);
    let bias_out = Self::apply_bias(clipper_out, bias);
    let tone_out = self.tone.process(bias_out, tone);
    tone_out * volume
  }

  fn apply_bias(input: f32, bias: f32) -> f32 {
    if input < 0. {
      input * (1. - bias)
    } else {
      input
    }
  }
}
