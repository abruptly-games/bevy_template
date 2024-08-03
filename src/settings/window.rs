use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct WindowSettings {
  pub present_mode: PresentMode,
  pub mode: WindowMode,
  #[cfg(not(target_arch = "wasm32"))]
  pub resolution: (f32, f32),
}

impl Default for WindowSettings {
  fn default() -> Self {
    WindowSettings {
      present_mode: PresentMode::AutoVsync,
      mode: WindowMode::Windowed,
      #[cfg(not(target_arch = "wasm32"))]
      resolution: (800.0, 600.0),
    }
  }
}

impl Settings for WindowSettings {
  const NAME: &'static str = "window";
}
