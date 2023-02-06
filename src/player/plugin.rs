use bevy::prelude::{App, Plugin};
pub struct All;

impl Plugin for All {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(super::systems::add_systems())
      .add_plugin(super::controller::GamepadPlugin);
  }
}
