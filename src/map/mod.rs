use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use std::collections::HashSet;

pub mod plugin;
pub mod systems;

const ASPECT_RATIO: f32 = 16. / 9.;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
  pub collider: Collider,
  pub rigid_body: RigidBody,
  pub velocity: Velocity,
  pub rotation_constraints: LockedAxes,
  pub friction: Friction,
  pub restitution: Restitution,
  pub mass_properties: ColliderMassProperties,
  pub force: ExternalForce,
}

impl From<EntityInstance> for ColliderBundle {
  fn from(entity_instance: EntityInstance) -> ColliderBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    match entity_instance.identifier.as_ref() {
      // "Player" | "Enemy" => ColliderBundle {
      "Player" => ColliderBundle {
        collider: Collider::ball(16.),
        rigid_body: RigidBody::KinematicPositionBased,
        rotation_constraints,
        ..Default::default()
      },
      "Enemy" => ColliderBundle {
        collider: Collider::ball(50.),
        rigid_body: RigidBody::KinematicPositionBased,
        rotation_constraints,
        ..Default::default()
      },
      _ => ColliderBundle::default(),
    }
  }
}

impl From<IntGridCell> for ColliderBundle {
  fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
    println!("int_grid_cell: {int_grid_cell:?}");

    if int_grid_cell.value == 1 {
      ColliderBundle {
        collider: Collider::cuboid(8., 8.),
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        ..Default::default()
      }
    } else {
      ColliderBundle::default()
    }
  }
}

#[derive(Clone, Default, Component, Resource)]
pub struct WallDetection;

#[derive(Component)]
pub struct WallSensor {
  pub wall_detection_entity: Entity,
  pub intersecting_wall_entities: HashSet<Entity>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
  wall: Wall,
}
