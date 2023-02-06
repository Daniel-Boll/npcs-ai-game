use bevy::prelude::*;
use leafwing_input_manager::{
  prelude::{ActionState, InputMap, SingleAxis},
  InputManagerBundle,
};

use super::{state_machine::TopDownAction, PLAYER_SPEED};

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(gamepad_connections)
      .add_system(on_change_gamepad);
  }
}

#[derive(Bundle)]
pub struct PlayerInput {
  #[bundle]
  input: InputManagerBundle<TopDownAction>,
}
impl Default for PlayerInput {
  fn default() -> Self {
    use TopDownAction::*;

    let mut input_map = InputMap::default();

    // basic movement
    input_map.insert(KeyCode::W, Up);
    input_map.insert(GamepadButtonType::DPadUp, Up);

    input_map.insert(KeyCode::S, Down);
    input_map.insert(GamepadButtonType::DPadDown, Down);

    input_map.insert(KeyCode::A, Left);
    input_map.insert(GamepadButtonType::DPadLeft, Left);

    input_map.insert(
      SingleAxis::symmetric(GamepadAxisType::LeftStickX, 0.1),
      Horizontal,
    );
    input_map.insert(
      SingleAxis::symmetric(GamepadAxisType::LeftStickY, 0.1),
      Vertical,
    );

    input_map.insert(KeyCode::D, Right);
    input_map.insert(GamepadButtonType::DPadRight, Right);

    // Jump
    input_map.insert(KeyCode::Space, TopDownAction::Dash);
    input_map.insert(GamepadButtonType::South, TopDownAction::Dash);

    input_map.insert(KeyCode::Return, TopDownAction::Pause);
    input_map.insert(GamepadButtonType::Start, TopDownAction::Pause);

    input_map.insert(KeyCode::I, TopDownAction::Menus);
    input_map.insert(GamepadButtonType::Select, TopDownAction::Menus);

    input_map.set_gamepad(Gamepad { id: 0 });

    Self {
      input: InputManagerBundle::<TopDownAction> {
        input_map,
        ..Default::default()
      },
    }
  }
}

pub fn transform_from_action(action_state: &ActionState<TopDownAction>, delta: f32) -> (f32, f32) {
  let x_transform = if action_state.pressed(TopDownAction::Horizontal) {
    action_state.action_data(TopDownAction::Horizontal).value
  } else if action_state.pressed(TopDownAction::Right) {
    action_state.clamped_value(TopDownAction::Right)
  } else if action_state.pressed(TopDownAction::Left) {
    -action_state.clamped_value(TopDownAction::Left)
  } else {
    0.0
  };

  let y_transform = if action_state.pressed(TopDownAction::Vertical) {
    action_state.action_data(TopDownAction::Vertical).value
  } else if action_state.pressed(TopDownAction::Up) {
    action_state.clamped_value(TopDownAction::Up)
  } else if action_state.pressed(TopDownAction::Down) {
    -action_state.clamped_value(TopDownAction::Down)
  } else {
    0.0
  };

  let x_transform = x_transform * PLAYER_SPEED * delta;
  let y_transform = y_transform * PLAYER_SPEED * delta;

  (x_transform, y_transform)
}

/// Simple resource to store the ID of the
/// connected gamepad. We need to know which
/// gamepad to use for player input.
#[derive(Debug, Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
  mut commands: Commands,
  my_gamepad: Option<Res<MyGamepad>>,
  mut gamepad_evr: EventReader<GamepadEvent>,
) {
  for GamepadEvent {
    gamepad,
    event_type,
  } in gamepad_evr.iter()
  {
    match event_type {
      GamepadEventType::Connected(_info) => {
        println!("New gamepad connected with ID: {:?}", gamepad.id);

        // if we don't have any gamepad yet, use this one
        if my_gamepad.is_none() {
          commands.insert_resource(MyGamepad(*gamepad));
        }
      }
      GamepadEventType::Disconnected => {
        println!("Lost gamepad connection with ID: {:?}", gamepad.id);

        // if it's the one we previously associated with the player, disassociate it:
        if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
          if old_id == gamepad {
            commands.remove_resource::<MyGamepad>();
          }
        }
      }
      // other events are irrelevant
      _ => {}
    }
  }
}

fn on_change_gamepad(
  gamepad: Option<Res<MyGamepad>>,
  mut input_map: Query<&mut InputMap<TopDownAction>>,
) {
  if let Some(gamepad) = gamepad {
    if gamepad.is_changed() {
      for mut map in input_map.iter_mut() {
        map.set_gamepad(gamepad.0);
      }
    }
  }
}
