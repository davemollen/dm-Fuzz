mod fir_filter;
use {crate::shared::float_ext::FloatExt, fir_filter::FirFilter};

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
    let upsampled = self.upsample_fir.process([input * OVERSAMPLE_FACTOR; 8]);
    let clipped = upsampled.map(|x| x.fast_tanh1());
    self.downsample_fir.process(clipped).into_iter().sum::<f32>() * 0.305496 // 0.610992 * 0.5
  }
}
