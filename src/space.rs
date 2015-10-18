use super::{Particle, Time};

#[allow(dead_code)]
pub enum Collision {
  Free,
  Wall { t: Time, prev: Particle, next: Particle },
  Bounce {
    t: Time,
    prev1: Particle, prev2: Particle,
    next1: Particle, next2: Particle
  }
}

pub trait Space {
  fn next_collision(&self) -> Collision;

  fn update(self, collision: Collision) -> Option<Self>;
}

