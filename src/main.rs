mod app_state;
mod constants;
mod dev_tools;
mod prelude;
mod settings;
mod utils;

mod main_menu;

use crate::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use main_menu::MainMenuPlugin;

fn main() -> AppExit {
  let mut app = App::new();

  // Add the background color to the App
  app.insert_resource(ClearColor(colors::GRAY_900));

  let audio_settings = AudioSettings::load();
  let window_settings = WindowSettings::load();

  let bevy_plugins = DefaultPlugins;

  // Change the default window settings
  let bevy_plugins = bevy_plugins.set(WindowPlugin {
    primary_window: Some(Window {
      title: "Bevy Game".into(),
      present_mode: window_settings.present_mode,
      mode: window_settings.mode,
      resolution: WindowResolution::new(window_settings.resolution.0, window_settings.resolution.1),
      prevent_default_event_handling: true,
      fit_canvas_to_parent: true,
      resizable: false,
      canvas: Some(window::CANVAS_ID.into()),
      ..Default::default()
    }),
    ..Default::default()
  });

  app.insert_resource::<WindowSettings>(window_settings);
  app.insert_resource::<AudioSettings>(audio_settings);

  let bevy_plugins = bevy_plugins.set(ImagePlugin::default_nearest());

  #[cfg(target_arch = "wasm32")]
  // Disable assets meta check on wasm to throw 4xx errors
  let bevy_plugins = bevy_plugins.set(AssetPlugin {
    meta_check: bevy::asset::AssetMetaCheck::Never,
    ..Default::default()
  });

  app.add_plugins(bevy_plugins);

  #[cfg(feature = "dev")]
  app.add_plugins(dev_tools::DevToolsPlugin);

  app.add_plugins(AppStatePlugin);

  // screens
  app.add_plugins(MainMenuPlugin::new(AppState::MainMenu));

  // Change the window when `Settings` resource changes
  app.add_systems(
    PostUpdate,
    window_settings_change
      .run_if(resource_changed::<WindowSettings>.and_then(not(resource_added::<WindowSettings>))),
  );

  app.run()
}

fn window_settings_change(mut query: Query<&mut Window>, window_settings: Res<WindowSettings>) {
  for mut window in &mut query {
    window.present_mode = window_settings.present_mode;
    window.mode = window_settings.mode;
    window.resolution =
      WindowResolution::new(window_settings.resolution.0, window_settings.resolution.1);
  }

  window_settings.save();
}
