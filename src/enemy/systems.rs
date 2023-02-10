use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::*;

use crate::{player::Player, utils::position::Pos};
use pathfinding::prelude::idastar;

use super::{
  state_machine::{Follow, Idle, Near},
  Enemy,
};

pub fn add_systems() -> SystemSet {
  SystemSet::new()
    .label("enemy")
    .with_system(
      spawn
        .label("enemy-spawn")
        .after("player")
        .after("player-spawn"),
    )
    .with_system(follow.label("enemy-follow").after("enemy-spawn"))
}

type PlayerGet<'a> = Entity;
type PlayerWhen = (Added<EntityInstance>, With<Player>, Without<Enemy>);

type EnemyGet<'a> = Entity;
type EnemyWhen = (Added<EntityInstance>, With<Enemy>, Without<Player>);

/// When the player is added through the ldtk bundle and the enemy is added through the ldtk bundle,
/// this system will add a follow component to the enemy and a state machine component to the enemy.
pub fn spawn(
  mut commands: Commands,
  players: Query<PlayerGet, PlayerWhen>,
  enemies: Query<EnemyGet, EnemyWhen>,
) {
  for player_entity in players.iter() {
    for enemy_entity in enemies.iter() {
      let follow_speed = 100.;
      let follow_distance = 100.;

      let near_player = Near::new(player_entity, follow_distance);

      commands
        .entity(enemy_entity)
        .insert((StateMachine::new(Idle)
          // Idle --(near_player)-> Follow
          .trans::<Idle>(near_player, Follow::new(player_entity, follow_speed))
          // Follow --(!near_player)-> Idle
          .trans::<Follow>(NotTrigger(near_player), Idle),));
    }
  }
}

/// When the enemy has a follow component, this system will move the enemy towards the target.
/// This function runs every tick.
// fn follow(
//   entities_with_transform: Query<&Transform>,
//   mut entities_with_kinematic_controller: Query<
//     (&mut KinematicCharacterController, &Transform),
//     With<Velocity>,
//   >,
//   follows: Query<(Entity, &Follow), With<Enemy>>,
//   time: Res<Time>,
// ) {
//   for (entity, follow) in &follows {
//     let target_position = entities_with_transform
//       .get(follow.target)
//       .expect("Enemy has no target")
//       .translation;
//     let enemy_controller = &mut entities_with_kinematic_controller
//       .get_mut(entity)
//       .expect("Enemy has no controller");
//
//     // In the enemy position get only the translation
//     let (
//       enemy_controller,
//       Transform {
//         translation: enemy_position,
//         ..
//       },
//     ) = enemy_controller;
//
//     // Calculate the vector to steer the enemy towards the target
//     let desired_translation = (target_position - *enemy_position)
//       .normalize_or_zero()
//       .truncate()
//       * time.delta_seconds()
//       * 100.;
//
//     enemy_controller.translation = match enemy_controller.translation {
//       Some(translation) => Some(translation + desired_translation),
//       None => Some(desired_translation),
//     };
//   }
// }

/// When the enemy has a follow component, this system will move the enemy towards the target using
/// A* pathfinding. This function runs every tick.
fn follow(
  mut enemy_entity_controller: Query<
    (&mut KinematicCharacterController, &Transform),
    (With<Velocity>, With<Enemy>),
  >,
  follows: Query<(Entity, &Follow), With<Enemy>>,
  time: Res<Time>,
  level_query: Query<&Handle<LdtkLevel>>,
  ldtk_levels: Res<Assets<LdtkLevel>>,
  level_selection: Res<LevelSelection>,
) {
  for level_handle in level_query.iter() {
    let level = &ldtk_levels
      .get(level_handle)
      .expect("Level not found")
      .level;

    if !level_selection.is_match(&0, level) {
      continue;
    }

    let mut walls: Vec<Pos> = Vec::new();
    let mut player_grid_position: Option<Pos> = None;
    let mut enemy_grid_position: Option<Pos> = None;

    for layer_instance in level
      .layer_instances
      .as_ref()
      .expect("No layer instance")
      .iter()
    {
      let LayerInstance {
        identifier,
        auto_layer_tiles,
        entity_instances,
        ..
      } = layer_instance;

      if identifier == "Entities" {
        for entity_instance in entity_instances {
          let EntityInstance {
            grid, identifier, ..
          } = entity_instance;

          let pos = Pos(grid[0], grid[1]);

          match identifier.as_ref() {
            "Player" => {
              player_grid_position = Some(pos);
            }
            "Enemy" => {
              enemy_grid_position = Some(pos);
            }
            _ => {}
          }
        }
      }

      walls = if identifier == "Walls" {
        auto_layer_tiles
          .iter()
          .map(|grid_tile| Pos(grid_tile.px[0] / 16, grid_tile.px[1] / 16))
          .collect()
      } else {
        walls
      }
    }

    let player_grid_position = player_grid_position.expect("Player not found");

    let path = idastar(
      &enemy_grid_position.expect("Enemy not found"),
      |p| p.successors(&walls),
      |p| p.manhattan_distance(&player_grid_position),
      |p| *p == player_grid_position,
    );

    // Steer the enemy towards the target
    if let Some(path) = path {
      let (
        mut enemy_controller,
        Transform {
          translation: enemy_position,
          ..
        },
      ) = enemy_entity_controller.single_mut();

      // TODO: Check if the enemy grid position is indeed updating when the enemy moves
      // if 👆 so then I will need to compare the path with the current enemy grid position
      // and get the next one
      let target_position = Vec3::new(
        path.0[0].0 as f32 * 8.,
        path.0[0].1 as f32 * 8.,
        enemy_position.z,
      );

      let desired_translation = (target_position - *enemy_position)
        .normalize_or_zero()
        .truncate()
        * time.delta_seconds()
        * 100.;

      enemy_controller.translation = match enemy_controller.translation {
        Some(translation) => Some(translation + desired_translation),
        None => Some(desired_translation),
      };
    }
  }
}
