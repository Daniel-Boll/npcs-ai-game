use bevy::prelude::*;
use seldom_state::prelude::Trigger;

#[derive(Clone, Copy, FromReflect, Reflect)]
pub struct Near {
  target: Entity,
  range: f32,
}

impl Near {
  pub fn new(target: Entity, range: f32) -> Self {
    Self { target, range }
  }
}

impl Trigger for Near {
  // mut transforms: Query<(&mut Transform, &Handle<ColorMaterial>)>,
  type Param<'w, 's> = (Query<'w, 's, &'static Transform>, Res<'s, Time>);

  fn trigger(&self, entity: Entity, (transforms, _time): &Self::Param<'_, '_>) -> bool {
    // Find the displacement between the target and this entity
    let delta = transforms.get(self.target).unwrap().translation
      - transforms.get(entity).unwrap().translation;

    // Check if the distance is less than the range
    delta.length() < self.range
  }
}

// Entities is the `BackToInitial` state should move towards the initial position at the given speed
#[derive(Clone, Component, Reflect, Debug)]
#[component(storage = "SparseSet")]
pub struct BackToInitial {
  pub target: Entity,
  pub initial_position: Vec2,
}

impl BackToInitial {
  pub fn new(target: Entity, initial_position: Vec2) -> Self {
    Self {
      target,
      initial_position,
    }
  }
}

// Entities in the `Idle` state should do nothing
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;

// Entities is the `Follow` state should move towards the given entity at the given speed
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Follow {
  pub target: Entity,
  pub speed: f32,
}

impl Follow {
  pub fn new(target: Entity, speed: f32) -> Self {
    Self { target, speed }
  }
}
