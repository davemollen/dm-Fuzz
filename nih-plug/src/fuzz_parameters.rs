use nih_plug::{
  formatters::{s2v_f32_percentage, v2s_f32_percentage},
  prelude::{FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

use crate::editor;

#[derive(Params)]
pub struct FuzzParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "pre_filter"]
  pub pre_filter: FloatParam,

  #[id = "gain"]
  pub gain: FloatParam,

  #[id = "bias"]
  pub bias: FloatParam,

  #[id = "tone"]
  pub tone: FloatParam,

  #[id = "volume"]
  pub volume: FloatParam,
}

impl Default for FuzzParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      pre_filter: FloatParam::new("Pre-filter", 0.15, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      gain: FloatParam::new("Gain", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      bias: FloatParam::new("Bias", 1.0, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      tone: FloatParam::new("Tone", 0.7, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      volume: FloatParam::new("Volume", 0.5, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
