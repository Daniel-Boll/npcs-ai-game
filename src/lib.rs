pub mod enemy;
pub mod map;
pub mod player;
pub mod utils;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  AssetLoading,
  Playing,
}
