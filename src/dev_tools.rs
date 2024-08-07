use crate::prelude::*;
use bevy::dev_tools::states::log_transitions;
use bevy::prelude::*;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
  fn build(&self, app: &mut App) {
    // Track all [`AppState`] transitions
    app.add_systems(Update, log_transitions::<AppState>);
  }
}
