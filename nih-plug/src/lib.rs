use fuzz::Fuzz;
use nih_plug::prelude::*;
use std::sync::Arc;
mod fuzz_parameters;
use fuzz_parameters::FuzzParameters;
mod editor;

struct DmFuzz {
  params: Arc<FuzzParameters>,
  fuzz: Fuzz,
}

impl DmFuzz {
  fn get_params(&self) -> (f32, f32, f32, f32, f32) {
    let gain = self.params.gain.value();

    (
      Fuzz::map_filter_param(self.params.pre_filter.value()),
      gain * gain * 2511.886432 + 1.,
      self.params.bias.value(),
      Fuzz::map_filter_param(self.params.tone.value() + 0.5),
      self.params.volume.value(),
    )
  }
}

impl Default for DmFuzz {
  fn default() -> Self {
    let params = Arc::new(FuzzParameters::default());
    Self {
      params: params.clone(),
      fuzz: Fuzz::new(44100.),
    }
  }
}

impl Plugin for DmFuzz {
  const NAME: &'static str = "dm-Fuzz";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Fuzz";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.fuzz = Fuzz::new(buffer_config.sample_rate);
    let (pre_filter, gain, bias, tone, volume) = self.get_params();
    self
      .fuzz
      .initialize_params(pre_filter, gain, bias, tone, volume);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (pre_filter, gain, bias, tone, volume) = self.get_params();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      *sample = self
        .fuzz
        .process(*sample, pre_filter, gain, bias, tone, volume);
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmFuzz {
  const CLAP_ID: &'static str = "dm-Fuzz";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A fuzz plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmFuzz {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Fuzz.........";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmFuzz);
nih_export_vst3!(DmFuzz);
