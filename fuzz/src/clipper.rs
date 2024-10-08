mod fir_filter;
use {
  fir_filter::FirFilter,
  std::simd::{f32x8, num::SimdFloat, StdFloat},
};

const OVERSAMPLE_FACTOR: f32 = 8.;

pub struct Clipper {
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      upsample_fir: FirFilter::new(),
      downsample_fir: FirFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let upsampled = self.upsample_fir.upsample(input);
    let clipped = Self::clip(upsampled);
    self.downsample_fir.downsample(clipped) * 0.5
  }

  fn clip(x: f32x8) -> f32x8 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let a = x + f32x8::splat(0.16489087) * x3 + f32x8::splat(0.00985468) * x5;
    a / (f32x8::splat(1.0) + a * a).sqrt()
  }
}
