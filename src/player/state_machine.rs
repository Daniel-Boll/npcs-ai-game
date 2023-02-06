use leafwing_input_manager::Actionlike;

// NOTE: I think It would be wiser to split the actions into two enums, one for
// the controller, one for the actions and other to the menu perhaps 🤔
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum TopDownAction {
  // Controller (keyboard)
  Right,
  Left,
  Down,
  Up,

  // Controller (gamepad)
  Horizontal,
  Vertical,

  // Combat actions
  Shoot,
  Dash,

  // Menu actions
  Pause,
  Menus,
}
