use serde::{Deserialize, Serialize};

mod audio;
mod window;

pub use audio::AudioSettings;
pub use window::WindowSettings;

pub trait Settings {
  const NAME: &'static str;

  #[cfg(not(target_arch = "wasm32"))]
  fn get_settings_path() -> std::path::PathBuf {
    use crate::APP_NAME;

    let settings_path = format!("{}/{}.ron", APP_NAME, Self::NAME);

    #[cfg(not(feature = "dev"))]
    {
      let home_dir = dirs::data_dir().unwrap();
      std::path::PathBuf::from(home_dir.join(settings_path))
    }

    #[cfg(feature = "dev")]
    std::path::PathBuf::from(settings_path)
  }

  fn load() -> Self
  where
    Self: Sized + Default + for<'a> Deserialize<'a> + Serialize,
  {
    let settings_path = Self::get_settings_path();

    let settings: Self = match std::fs::read_to_string(settings_path) {
      Ok(settings) => ron::from_str(&settings).unwrap(),
      Err(_) => {
        let settings = Self::default();
        settings.save();
        settings
      }
    };

    settings
  }

  #[cfg(target_arch = "wasm32")]
  fn load() -> Self
  where
    Self: Sized + Default + for<'a> Deserialize<'a> + Serialize,
  {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let settings: Self = match local_storage.get(Self::NAME).unwrap() {
      Some(settings) => serde_json::from_str(&settings).unwrap(),
      None => {
        let settings = Self::default();
        settings.save();
        settings
      }
    };

    settings
  }

  fn save(&self)
  where
    Self: Default + Serialize,
  {
    let settings_path = Self::get_settings_path();
    let settings = ron::to_string(self).unwrap();

    std::fs::create_dir_all(settings_path.parent().unwrap()).unwrap();
    std::fs::write(settings_path, settings).unwrap();
  }

  #[cfg(target_arch = "wasm32")]
  pub fn save(&self)
  where
    Self: Default + Serialize,
  {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let settings = serde_json::to_string(self).unwrap();
    local_storage.set(Self::NAME, &settings).unwrap();
  }
}
