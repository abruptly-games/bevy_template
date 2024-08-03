use bevy::prelude::*;

use crate::prelude::*;

pub mod settings;

pub struct MainMenuPlugin<S: States> {
  pub state: S,
}

impl<S: States> Plugin for MainMenuPlugin<S> {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(self.state.clone()), init_main_menu);
  }
}

impl<S: States> MainMenuPlugin<S> {
  pub fn new(state: S) -> Self {
    Self { state }
  }
}

fn init_main_menu(mut commands: Commands) {
  commands.spawn((StateDespawnMarker, Camera2dBundle::default()));
}
