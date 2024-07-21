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

  pub fn map_params(
    &self,
    pre_filter: f32,
    gain: f32,
    bias: f32,
    tone: f32,
    volume: f32,
    style: i32,
  ) -> (f32, f32, f32, f32, f32, i32) {
    (
      Self::map_filter_param(pre_filter),
      gain * gain * gain * 2511.886432 + 1.,
      bias,
      Self::map_filter_param(tone + 0.5),
      volume * volume,
      style,
    )
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
    style: i32,
  ) -> f32 {
    let (pre_filter, gain, bias, tone, volume) = self
      .smooth_parameters
      .process(pre_filter, gain, bias, tone, volume);

    let pre_filter_out = self.pre_filter.process(input, pre_filter);
    let clipper_out = self.clip(pre_filter_out, gain, bias, style);
    let tone_out = self.tone.process(clipper_out, tone);
    tone_out * volume
  }

  fn clip(&mut self, input: f32, gain: f32, bias: f32, style: i32) -> f32 {
    match style {
      0 => {
        let clipper_out = self.clipper.process(input * gain);
        if clipper_out < 0. {
          clipper_out * (1. - bias)
        } else {
          clipper_out
        }
      }
      _ => {
        let scaled_input = input * gain;
        let bias = bias.powf(0.33333);
        let clipper_input = scaled_input + scaled_input.abs() * bias;
        self.clipper.process(clipper_input / 2_f32.powf(bias))
      }
    }
  }

  fn map_filter_param(filter: f32) -> f32 {
    filter * filter * 0.175438596
  }
}
