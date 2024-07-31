mod app_state;
mod constants;
mod dev_tools;
mod prelude;
mod utils;

mod main_menu;

use crate::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

fn main() -> AppExit {
  let mut app = App::new();

  // Add the background color to the App
  app.insert_resource(ClearColor(colors::GRAY_900));

  let bevy_plugins = DefaultPlugins;

  // Change the default window settings
  let bevy_plugins = bevy_plugins.set(WindowPlugin {
    primary_window: Some(Window {
      title: "Bevy Game".into(),
      present_mode: window::PRESENT_MODE,
      mode: window::MODE,
      resolution: WindowResolution::new(window::RESOLUTIONS[0].0, window::RESOLUTIONS[0].1),
      prevent_default_event_handling: true,
      fit_canvas_to_parent: true,
      canvas: Some(window::CANVAS_ID.into()),
      ..Default::default()
    }),
    ..Default::default()
  });

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

  app.run()
}
