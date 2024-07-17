extern crate fuzz;
extern crate lv2;
use fuzz::Fuzz;
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  pre_filter: InputPort<Control>,
  gain: InputPort<Control>,
  bias: InputPort<Control>,
  tone: InputPort<Control>,
  volume: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Fuzz")]
struct DmFuzz {
  fuzz: Fuzz,
  is_active: bool,
}

impl Plugin for DmFuzz {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      fuzz: Fuzz::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let pre_filter = Fuzz::map_filter_param(*ports.pre_filter);
    let gain = *ports.gain * *ports.gain * *ports.gain * 2511.886432 + 1.;
    let bias = *ports.bias;
    let tone = Fuzz::map_filter_param(*ports.tone * 0.5);
    let volume = *ports.volume;

    if !self.is_active {
      self
        .fuzz
        .initialize_params(pre_filter, gain, bias, tone, volume);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self
        .fuzz
        .process(*input, pre_filter, gain, bias, tone, volume);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmFuzz);
