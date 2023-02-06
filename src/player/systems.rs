use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::{KinematicCharacterController, Velocity};
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::ActionState;

use crate::GameState;

use super::{controller::transform_from_action, state_machine::TopDownAction, Player};

pub fn add_systems() -> SystemSet {
  SystemSet::new()
    .label("player")
    // .with_system(spawn.label("player-spawn"))
    .with_system(
      movement
        .run_in_state(GameState::Playing)
        .label("player-movement"), // .after("player-spawn"),
    )
}

type PlayerQueryGet = (Entity, &'static Transform);
type PlayerQueryWhen = (Added<EntityInstance>, With<Player>);
pub fn spawn(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  players: Query<PlayerQueryGet, PlayerQueryWhen>,
) {
  let mesh = meshes.add(Mesh::from(shape::Capsule {
    radius: 12.0,
    depth: 24.0,
    ..default()
  }));
  let material = materials.add(ColorMaterial::from(Color::hex("1fa9f4").unwrap()));
  for (player, transform) in players.iter() {
    commands.entity(player).insert(MaterialMesh2dBundle {
      mesh: mesh.clone().into(),
      material: material.clone(),
      transform: *transform,
      ..default()
    });
  }
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
