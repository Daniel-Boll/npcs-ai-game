#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

impl Pos {
  pub fn manhattan_distance(&self, other: &Pos) -> u32 {
    ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as u32
  }

  pub fn successors(&self, walls: &[Pos]) -> Vec<(Pos, u32)> {
    vec![
      Pos(self.0 - 1, self.1),
      Pos(self.0 + 1, self.1),
      Pos(self.0, self.1 - 1),
      Pos(self.0, self.1 + 1),
    ]
    .into_iter()
    .filter(|p| !walls.contains(p))
    .map(|p| (p, 1))
    .collect()
  }
}
