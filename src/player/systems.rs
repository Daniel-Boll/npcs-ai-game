use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController, Velocity};
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::ActionState;

use crate::GameState;

use super::{controller::transform_from_action, state_machine::TopDownAction};

pub fn add_systems() -> SystemSet {
  SystemSet::new().label("player").with_system(
    movement
      .run_in_state(GameState::Playing)
      .label("player-movement"), // .after("player-spawn"),
  )
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
