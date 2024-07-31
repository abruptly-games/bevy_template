use crate::prelude::*;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Resource)]
pub struct Settings {
  pub present_mode: PresentMode,
  pub mode: WindowMode,
  #[cfg(not(target_arch = "wasm32"))]
  pub resolution: (f32, f32),
}

impl Settings {
  #[cfg(not(target_arch = "wasm32"))]
  fn get_settings_path() -> std::path::PathBuf {
    let settings_path = format!("{}/settings.ron", APP_NAME);

    #[cfg(not(feature = "dev"))]
    {
      let home_dir = dirs::data_dir().unwrap();
      std::path::PathBuf::from(home_dir.join(settings_path))
    }

    #[cfg(feature = "dev")]
    std::path::PathBuf::from(settings_path)
  }

  pub fn load() -> Self {
    let settings_path = Self::get_settings_path();

    let settings: Settings = match std::fs::read_to_string(settings_path) {
      Ok(settings) => ron::from_str(&settings).unwrap(),
      Err(_) => {
        let settings = Settings::default();
        settings.save();
        settings
      }
    };

    settings
  }

  #[cfg(target_arch = "wasm32")]
  pub fn load() -> Self {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let settings: Settings = match local_storage.get("settings").unwrap() {
      Some(settings) => serde_json::from_str(&settings).unwrap(),
      None => {
        let settings = Settings::default();
        settings.save();
        settings
      }
    };

    settings
  }

  #[cfg(target_os = "android")]
  pub fn load() -> Self {
    unimplemented!("Settings loading is not implemented for Android");
  }

  #[cfg(target_os = "ios")]
  pub fn load() -> Self {
    unimplemented!("Settings loading is not implemented for iOS");
  }

  pub fn save(&self) {
    let settings_path = Self::get_settings_path();

    let settings = ron::to_string(self).unwrap();
    std::fs::write(settings_path, settings).unwrap();
  }

  #[cfg(target_arch = "wasm32")]
  pub fn save(&self) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let settings = serde_json::to_string(self).unwrap();
    local_storage.set("settings", &settings).unwrap();
  }

  #[cfg(target_os = "android")]
  pub fn save(&self) {
    // see https://github.com/jinleili/bevy-in-app
    unimplemented!("Settings saving is not implemented for Android");
  }

  #[cfg(target_os = "ios")]
  pub fn save(&self) {
    // see https://github.com/jinleili/bevy-in-app
    unimplemented!("Settings saving is not implemented for iOS");
  }
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      present_mode: PresentMode::AutoVsync,
      mode: WindowMode::Windowed,
      #[cfg(not(target_arch = "wasm32"))]
      resolution: (800.0, 600.0),
    }
  }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut App) {
    let settings = Settings::load();
    app.insert_resource::<Settings>(settings);

    app.add_systems(OnEnter(MainMenuSubState::Settings), init_settings);
  }
}

fn init_settings(mut _commands: Commands) {
  println!("Settings");
}
