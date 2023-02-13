use bevy::prelude::*;
use bevy_ecs_ldtk::{utils::translation_to_grid_coords, GridCoords};
use bevy_rapier2d::prelude::{KinematicCharacterController, Velocity};
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::ActionState;

use crate::GameState;

use super::{controller::transform_from_action, state_machine::TopDownAction, Player};

pub fn add_systems() -> SystemSet {
  SystemSet::new()
    .label("player")
    .with_system(
      movement
        .run_in_state(GameState::Playing)
        .label("player-movement"), // .after("player-spawn"),
    )
    .with_system(
      update_grid_coords_from_player
        .run_in_state(GameState::Playing)
        .label("player-grid-coords"), // .after("player-spawn"),
    )
    // .with_system(
    //   debug_user_grid_coordinates
    //     .run_in_state(GameState::Playing)
    //     .label("player-debug-grid-coords"), // .after("player-spawn"),
    // )
}

pub fn movement(
  mut controllers: Query<
    (
      &ActionState<TopDownAction>,
      &mut KinematicCharacterController,
    ),
    With<Velocity>,
  >,
  time: Res<Time>,
) {
  for (action_state, mut controller) in controllers.iter_mut() {
    let (x_transform, y_transform) = transform_from_action(action_state, time.delta_seconds());

    controller.translation = match controller.translation {
      Some(mut v) => {
        v.x = x_transform;
        v.y = y_transform;
        Some(v)
      }
      None => Some(Vec2::new(x_transform, y_transform)),
    };
  }
}

/// The initial transform of the player is (19.0, 330.0, 3.0) and is equivalent to (0, 0) in grid coordinates.
fn update_grid_coords_from_player(mut player: Query<(&Transform, &mut GridCoords), With<Player>>) {
  for (transform, mut grid_coords) in player.iter_mut() {
    let grid = translation_to_grid_coords(transform.translation.truncate(), (16, 16).into());

    grid_coords.x = grid.x;
    grid_coords.y = 16 - grid.y - 1;
  }
}

fn debug_user_grid_coordinates(mut player: Query<&GridCoords, With<Player>>) {
  for grid_coords in player.iter_mut() {
    println!("Player grid coordinates: {grid_coords:?}");
  }
}
