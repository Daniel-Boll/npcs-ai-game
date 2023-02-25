use astar_pathfinding::astar;
use bevy::prelude::*;
use bevy_ecs_ldtk::{
  prelude::*,
  utils::{grid_coords_to_translation, translation_to_grid_coords},
};
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::*;

use crate::{player::Player, utils::position::Pos};

use super::{
  state_machine::{BackToInitial, Follow, Idle, Near},
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
    .with_system(
      back_to_initial
        .label("enemy-back-to-initial")
        .after("enemy-follow"),
    )
    .with_system(follow.label("enemy-follow").after("enemy-spawn"))
    .with_system(
      update_grid_coords_from_enemy
        .label("enemy-grid-coords")
        .after("enemy-spawn"),
    )
  // .with_system(
  //   debug_enemy_grid_coordinates
  //     .label("enemy-debug-grid-coords")
  //     .after("enemy-spawn"),
  // )
}

type PlayerGet<'a> = Entity;
type PlayerWhen = (Added<EntityInstance>, With<Player>, Without<Enemy>);

type EnemyGet<'a> = (Entity, &'static Transform);
type EnemyWhen = (Added<EntityInstance>, With<Enemy>, Without<Player>);

/// When the player is added through the ldtk bundle and the enemy is added through the ldtk bundle,
/// this system will add a follow component to the enemy and a state machine component to the enemy.
pub fn spawn(
  mut commands: Commands,
  players: Query<PlayerGet, PlayerWhen>,
  enemies: Query<EnemyGet, EnemyWhen>,
) {
  for player_entity in players.iter() {
    for (enemy_entity, enemy_transform) in enemies.iter() {
      let follow_speed = 250.;
      let follow_distance = 100.;

      let mut enemy_initial_position =
        translation_to_grid_coords(enemy_transform.translation.truncate(), (16, 16).into());
      enemy_initial_position.y = 16 - enemy_initial_position.y - 1;
      // create a Vec2 from the enemy's initial position
      let initial_position = Vec2::new(
        enemy_initial_position.x as f32,
        enemy_initial_position.y as f32,
      );

      let near_player = Near::new(player_entity, follow_distance);

      commands
        .entity(enemy_entity)
        .insert((StateMachine::new(Idle)
          // Idle --(near_player)-> Follow
          .trans::<Idle>(near_player, Follow::new(player_entity, follow_speed))
          // Follow --(!near_player)-> Idle
          .trans::<Follow>(
            NotTrigger(near_player),
            BackToInitial::new(enemy_entity, initial_position),
          )
          // BackToInitial --(near_player)-> Follow
          .trans::<BackToInitial>(near_player, Follow::new(player_entity, follow_speed))
          // BackToInitial --(current_position == initial_position)-> Idle
          .trans::<BackToInitial>(DoneTrigger::Success, Idle),));
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

fn back_to_initial(
  mut commands: Commands,
  enemies_to_back: Query<(Entity, &BackToInitial), With<Enemy>>,
  mut movable_entities: Query<
    (&mut KinematicCharacterController, &GridCoords, &Transform),
    With<Velocity>,
  >,
  level_query: Query<&Handle<LdtkLevel>>,
  ldtk_levels: Res<Assets<LdtkLevel>>,
  level_selection: Res<LevelSelection>,
  time: Res<Time>,
) {
  if enemies_to_back.iter().count() == 0 {
    return;
  }
  let (
    target,
    &BackToInitial {
      initial_position, ..
    },
    ..,
  ) = enemies_to_back.single();

  let (
    mut enemy_controller,
    enemy_grid_position,
    &Transform {
      translation: enemy_position,
      ..
    },
  ) = movable_entities.get_mut(target).unwrap();
  let enemy_grid_position = Pos(enemy_grid_position.x, enemy_grid_position.y);
  let initial_grid_position = Pos(initial_position.x as i32, initial_position.y as i32);

  for level_handle in level_query.iter() {
    let level = &ldtk_levels
      .get(level_handle)
      .expect("Level not found")
      .level;

    if !level_selection.is_match(&0, level) {
      continue;
    }

    let mut walls: Vec<Pos> = Vec::new();

    for layer_instance in level
      .layer_instances
      .as_ref()
      .expect("No layer instance")
      .iter()
    {
      let LayerInstance {
        identifier,
        auto_layer_tiles,
        ..
      } = layer_instance;

      walls = if identifier == "Walls" {
        auto_layer_tiles
          .iter()
          .map(|grid_tile| Pos(grid_tile.px[0] / 16, grid_tile.px[1] / 16))
          .collect()
      } else {
        walls
      }
    }

    let path = astar(
      &enemy_grid_position,
      |p| p.successors(&walls),
      |p| p.manhattan_distance(&initial_grid_position),
      |p| *p == initial_grid_position,
    );

    if path.is_none() {
      return;
    }

    let path = path
      .unwrap()
      .0
      .iter()
      .map(|p| Pos(p.0, p.1))
      .collect::<Vec<Pos>>();

    // Find the next position. Find the index of the enemy current position and get the next one,
    // if the enemy is not in the path, get the first position.
    let next_tile_index = path
      .iter()
      .position(|p| *p == enemy_grid_position)
      .unwrap_or(0)
      + 1;

    let path = path.split_at(next_tile_index).1;

    if path.len() == 0 {
      commands.entity(target).insert(Done::Success);
      return;
    }

    let next_tile = GridCoords {
      x: path[0].0,
      y: 16 - path[0].1 - 1,
    };

    // Steer the enemy towards the target
    let target_position = grid_coords_to_translation(next_tile, (16, 16).into());

    let desired_translation = (target_position - enemy_position.truncate()).normalize_or_zero()
      * time.delta_seconds()
      * 30.;

    enemy_controller.translation = match enemy_controller.translation {
      Some(translation) => Some(translation + desired_translation),
      None => Some(desired_translation),
    };
  }
}

/// When the enemy has a follow component, this system will move the enemy towards the target using
/// A* pathfinding. This function runs every tick.
fn follow(
  follows: Query<(Entity, &Follow), With<Enemy>>,
  mut movable_entities: Query<
    (&mut KinematicCharacterController, &GridCoords, &Transform),
    With<Velocity>,
  >,
  level_query: Query<&Handle<LdtkLevel>>,
  ldtk_levels: Res<Assets<LdtkLevel>>,
  level_selection: Res<LevelSelection>,
  time: Res<Time>,
) {
  if follows.iter().count() == 0 {
    return;
  }

  for level_handle in level_query.iter() {
    let level = &ldtk_levels
      .get(level_handle)
      .expect("Level not found")
      .level;

    if !level_selection.is_match(&0, level) {
      continue;
    }

    let mut walls: Vec<Pos> = Vec::new();

    for layer_instance in level
      .layer_instances
      .as_ref()
      .expect("No layer instance")
      .iter()
    {
      let LayerInstance {
        identifier,
        auto_layer_tiles,
        ..
      } = layer_instance;

      walls = if identifier == "Walls" {
        auto_layer_tiles
          .iter()
          .map(|grid_tile| Pos(grid_tile.px[0] / 16, grid_tile.px[1] / 16))
          .collect()
      } else {
        walls
      }
    }

    let (
      enemy_entity,
      &Follow {
        target: player_entity,
        ..
      },
    ) = follows.single();

    let [
      (
        mut enemy_controller,
        enemy_grid_position,
        &Transform {
          translation: enemy_position,
          ..
        },
      ),
      (_, player_grid_position, _),
    ] = movable_entities
      .get_many_mut([enemy_entity, player_entity])
      .unwrap();

    let enemy_grid_position = Pos(enemy_grid_position.x, enemy_grid_position.y);
    let player_grid_position = Pos(player_grid_position.x, player_grid_position.y);

    let path = astar(
      &enemy_grid_position,
      |p| p.successors(&walls),
      |p| p.manhattan_distance(&player_grid_position),
      |p| *p == player_grid_position,
    );

    if path.is_none() {
      return;
    }

    let path = path
      .unwrap()
      .0
      .iter()
      .map(|p| Pos(p.0, p.1))
      .collect::<Vec<Pos>>();

    // Find the next position. Find the index of the enemy current position and get the next one,
    // if the enemy is not in the path, get the first position.
    let next_tile_index = path
      .iter()
      .position(|p| *p == enemy_grid_position)
      .unwrap_or(0)
      + 1;
    let path = path.split_at(next_tile_index).1;

    let next_tile = GridCoords {
      x: path[0].0,
      y: 16 - path[0].1 - 1,
    };

    // Steer the enemy towards the target
    let target_position = grid_coords_to_translation(next_tile, (16, 16).into());

    let desired_translation = (target_position - enemy_position.truncate()).normalize_or_zero()
      * time.delta_seconds()
      * 30.;

    enemy_controller.translation = match enemy_controller.translation {
      Some(translation) => Some(translation + desired_translation),
      None => Some(desired_translation),
    };
  }
}

fn update_grid_coords_from_enemy(
  mut player: Query<(&Transform, &mut GridCoords), (With<Enemy>, Without<Player>)>,
) {
  for (transform, mut grid_coords) in player.iter_mut() {
    let grid = translation_to_grid_coords(transform.translation.truncate(), (16, 16).into());

    grid_coords.x = grid.x;
    grid_coords.y = 16 - grid.y - 1;
  }
}

// fn debug_enemy_grid_coordinates(mut player: Query<&GridCoords, With<Enemy>>) {
//   for grid_coords in player.iter_mut() {
//     println!("Enemy grid coordinates: {grid_coords:?}");
//   }
// }
