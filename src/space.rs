use std::mem;
use std::slice;
use std::cmp::{PartialOrd, Ord, Ordering};
use super::{custom_float, FloatOps, Particle, Time};

#[derive(Debug, PartialEq)]
pub enum Collision {
  Free,
  Wall { t: Time, prev: Particle, next: Particle },
  Bounce {
    t: Time,
    prev1: Particle, prev2: Particle,
    next1: Particle, next2: Particle
  }
}

impl Eq for Collision { }

fn t_flops(coll: &Collision) -> FloatOps {
  match coll {
    &Collision::Wall { t: Time(t), .. } => FloatOps(t),
    &Collision::Bounce { t: Time(t), .. } => FloatOps(t),
    &Collision::Free => FloatOps(custom_float::MAX),
  }
}

impl PartialOrd for Collision {
  fn partial_cmp(&self, other: &Collision) -> Option<Ordering> {
    t_flops(self).partial_cmp(&t_flops(other))
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

  fn next_collision(&self) -> Collision;

  fn update(&self, collision: Collision) -> Option<Self>;

  fn iterate(&self) -> Option<Self> {
    let coll = self.next_collision();
    self.update(coll)
  }
}

pub struct SpaceIterator<S: Space> {
  space: Option<S>
}

impl<S: Space> SpaceIterator<S> {
  pub fn new(s: S) -> SpaceIterator<S> {
    SpaceIterator {
      space: Some(s),
    }
  }
}

impl<S: Space> Iterator for SpaceIterator<S> {
  type Item = S;
  fn next(&mut self) -> Option<S> {
    let mut s_new = self.space.as_ref().and_then(|s| s.iterate());
    mem::swap(&mut s_new, &mut self.space);
    s_new
  }
}

