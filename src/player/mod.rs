use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::map::{ColliderBundle, WallDetection};

use self::controller::PlayerInput;

pub mod controller;
pub mod plugin;
pub mod state_machine;
pub mod systems;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Player;

pub const PLAYER_SPEED: f32 = 300.0;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
  #[from_entity_instance]
  #[bundle]
  pub collider_bundle: ColliderBundle,
  pub wall_detection: WallDetection,

  pub player: Player,
  pub controller: KinematicCharacterController,

  #[bundle]
  pub input: PlayerInput,

  #[sprite_bundle("player.png")]
  #[bundle]
  sprite_bundle: SpriteBundle,

  #[worldly]
  pub worldly: Worldly,

  #[from_entity_instance]
  entity_instance: EntityInstance,
}
