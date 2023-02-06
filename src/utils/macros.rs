#[macro_export]
macro_rules! add_all_systems {
  ($struct_name:ident) => {
    use bevy::prelude::{App, Plugin};
    pub struct $struct_name;

    impl Plugin for $struct_name {
      fn build(&self, app: &mut App) {
        app.add_system_set(super::systems::add_systems());
      }
    }
  };
}
