mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub pre_filter: LinearSmooth,
  pub gain: LinearSmooth,
  pub bias: LinearSmooth,
  pub tone: LinearSmooth,
  pub volume: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      pre_filter: LinearSmooth::new(sample_rate, 20.),
      gain: LinearSmooth::new(sample_rate, 20.),
      bias: LinearSmooth::new(sample_rate, 20.),
      tone: LinearSmooth::new(sample_rate, 20.),
      volume: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, pre_filter: f32, gain: f32, bias: f32, tone: f32, volume: f32) {
    let pre_filter = Self::map_filter_param(pre_filter);
    let gain = gain * gain * gain * 2511.886432 + 1.0;
    let tone = Self::map_filter_param(tone + 0.5);
    let volume = volume * volume;

    if self.is_initialized {
      self.pre_filter.set_target(pre_filter);
      self.gain.set_target(gain);
      self.bias.set_target(bias);
      self.tone.set_target(tone);
      self.volume.set_target(volume);
    } else {
      self.pre_filter.reset(pre_filter);
      self.gain.reset(gain);
      self.bias.reset(bias);
      self.tone.reset(tone);
      self.volume.reset(volume);
      self.is_initialized = true;
    }
  }

  fn map_filter_param(filter: f32) -> f32 {
    filter * filter * 0.175438596
  }
}
