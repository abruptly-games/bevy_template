use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct AudioSettings {
  pub master_volume: f32,
  pub music_volume: f32,
  pub sfx_volume: f32,
}

impl Default for AudioSettings {
  fn default() -> Self {
    AudioSettings {
      master_volume: 1.0,
      music_volume: 1.0,
      sfx_volume: 1.0,
    }
  }
}

impl Settings for AudioSettings {
  const NAME: &'static str = "audio";
}
