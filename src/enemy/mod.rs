use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::map::ColliderBundle;

pub mod plugin;
pub mod state_machine;
pub mod systems;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Enemy;

pub const ENEMY_SPEED: f32 = 300.0;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
  #[from_entity_instance]
  #[bundle]
  pub collider_bundle: ColliderBundle,

  pub enemy: Enemy,
  pub controller: KinematicCharacterController,

  #[worldly]
  pub worldly: Worldly,

  #[sprite_sheet_bundle]
  #[bundle]
  sprite_bundle: SpriteSheetBundle,

  #[from_entity_instance]
  entity_instance: EntityInstance,

  #[grid_coords]
  grid_coords: GridCoords,
}
