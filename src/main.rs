use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use seldom_state::prelude::*;

use iyes_progress::{ProgressCounter, ProgressPlugin};
use npcs::{
  enemy::{self, state_machine::Near},
  map,
  player::{self, state_machine::TopDownAction},
  GameState,
};

#[derive(AssetCollection, Resource)]
struct ImageAssets {
  // #[asset(path = "map/map.ldtk")]
  #[asset(path = "map/sandbox.ldtk")]
  map: Handle<LdtkAsset>,
}

fn main() {
  let mut app = App::new();
  app.add_loopless_state(GameState::AssetLoading);

  LoadingState::new(GameState::AssetLoading)
    .continue_to_state(GameState::Playing)
    .with_collection::<ImageAssets>()
    .build(&mut app);

  app
    .add_plugins(DefaultPlugins)
    .add_plugin(ProgressPlugin::new(GameState::AssetLoading))
    .add_plugin(LdtkPlugin)
    .add_plugin(StateMachinePlugin)
    .add_plugin(TriggerPlugin::<Near>::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(InputManagerPlugin::<TopDownAction>::default())
    // Resources
    .insert_resource(LdtkSettings {
      level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
        load_level_neighbors: true,
      },
      set_clear_color: SetClearColor::FromLevelBackground,
      ..Default::default()
    })
    .insert_resource(LevelSelection::Index(0))
    // ============ Enter system ============
    .add_enter_system(GameState::Playing, init)
    // ============ (My scope) ============
    .add_plugin(player::plugin::All)
    .add_plugin(enemy::plugin::All)
    .add_plugin(map::plugin::All)
    // ============ Ldtk entity registry ============
    .register_ldtk_entity::<player::PlayerBundle>("Player")
    .register_ldtk_entity::<enemy::EnemyBundle>("Enemy")
    .register_ldtk_int_cell::<map::WallBundle>(1)
    // ============ Stage system ============
    .add_system_to_stage(CoreStage::PostUpdate, print_progress);

  app.run();
}

fn init(mut commands: Commands, images: Res<ImageAssets>) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn(LdtkWorldBundle {
    ldtk_handle: images.map.clone(),
    ..Default::default()
  });
}

fn print_progress(progress: Option<Res<ProgressCounter>>) {
  if let Some(progress) = progress {
    info!("Current progress: {:?}", progress.progress());
  }
}
