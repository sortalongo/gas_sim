use std::slice;
use std::cmp::{PartialOrd, Ord, Ordering};
use super::{custom_float, FloatOps, Particle, Time};

#[derive(Debug, Clone, PartialEq)]
pub enum Collision {
  Free,
  Wall { t: Time, prev: Particle, next: Particle },
  Bounce {
    t: Time,
    prev1: Particle, prev2: Particle,
    next1: Particle, next2: Particle
  }
}

impl Collision {
  pub fn t(&self) -> Time {
    match self {
      &Collision::Wall { t, .. } |
      &Collision::Bounce { t, .. } => t,
      _ => Time(custom_float::INFINITY),
    }
  }
  pub fn t_mut<'l>(&'l mut self) -> &'l mut Time {
    match self {
      &mut Collision::Wall { ref mut t, .. } |
      &mut Collision::Bounce { ref mut t, .. } => t,
      _ => {
        error!("Collision::t_unsafe called on Collision::Free");
        unreachable!()
      },
    }
  }
}

impl Eq for Collision { }

impl PartialOrd for Collision {
  fn partial_cmp(&self, other: &Collision) -> Option<Ordering> {
    self.t().partial_cmp(&other.t())
  }
}
impl Ord for Collision {
  fn cmp(&self, other: &Collision) -> Ordering {
    self.partial_cmp(other)
        .expect(&format!("Unable to compare collisions: {:?} and {:?}", self, other))
  }
}

pub trait Space: Sized {
  fn particles(&self) -> slice::Iter<Particle>;

  fn map_particles<F>(&self, f: F) -> Self
  where F: FnMut(&Particle) -> Particle;

  fn next_collision(&self) -> Collision;

  fn update(&self, collision: &Collision) -> Option<Self>;
}

