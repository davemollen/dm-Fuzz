mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub pre_filter: LinearSmooth,
  pub gain: LinearSmooth,
  pub bias: LinearSmooth,
  pub tone: LinearSmooth,
  pub volume: LinearSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pre_filter: LinearSmooth::new(20.0, sample_rate),
      gain: LinearSmooth::new(20.0, sample_rate),
      bias: LinearSmooth::new(20.0, sample_rate),
      tone: LinearSmooth::new(20.0, sample_rate),
      volume: LinearSmooth::new(20.0, sample_rate),
    }
  }

  pub fn set(&mut self, pre_filter: f32, gain: f32, bias: f32, tone: f32, volume: f32) {
    self
      .pre_filter
      .set_target(Self::map_filter_param(pre_filter));
    self.gain.set_target(gain * gain * gain * 2511.886432 + 1.0);
    self.bias.set_target(bias);
    self.tone.set_target(Self::map_filter_param(tone + 0.5));
    self.volume.set_target(volume * volume);
  }

  fn map_filter_param(filter: f32) -> f32 {
    filter * filter * 0.175438596
  }
}
