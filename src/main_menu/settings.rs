use bevy::prelude::*;

pub struct SettingsPlugin<S: SubStates> {
  pub sub_state: S,
}

impl<S: SubStates> Plugin for SettingsPlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(self.sub_state.clone()), init_settings);
  }
}

fn init_settings(mut _commands: Commands) {
  println!("Settings");
}
