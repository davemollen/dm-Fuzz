mod ramp_smooth;
use ramp_smooth::RampSmooth;

pub struct SmoothParameters {
  smooth_pre_filter: RampSmooth,
  smooth_gain: RampSmooth,
  smooth_bias: RampSmooth,
  smooth_tone: RampSmooth,
  smooth_volume: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_pre_filter: RampSmooth::new(sample_rate, 20.),
      smooth_gain: RampSmooth::new(sample_rate, 20.),
      smooth_bias: RampSmooth::new(sample_rate, 20.),
      smooth_tone: RampSmooth::new(sample_rate, 20.),
      smooth_volume: RampSmooth::new(sample_rate, 20.),
    }
  }

  pub fn initialize(&mut self, pre_filter: f32, gain: f32, bias: f32, tone: f32, volume: f32) {
    self.smooth_pre_filter.initialize(pre_filter);
    self.smooth_gain.initialize(gain);
    self.smooth_bias.initialize(bias);
    self.smooth_tone.initialize(tone);
    self.smooth_volume.initialize(volume);
  }

  pub fn process(
    &mut self,
    pre_filter: f32,
    gain: f32,
    bias: f32,
    tone: f32,
    volume: f32,
  ) -> (f32, f32, f32, f32, f32) {
    (
      self.smooth_pre_filter.process(pre_filter),
      self.smooth_gain.process(gain),
      self.smooth_bias.process(bias),
      self.smooth_tone.process(tone),
      self.smooth_volume.process(volume),
    )
  }
}
