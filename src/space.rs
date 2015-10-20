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

  fn update(&mut self, collision: Collision) -> Option<&mut Self>;

  fn iterate(&mut self) -> bool {
    let coll = self.next_collision();
    self.update(coll).is_some()
  }
}
/*
pub struct SpaceIterator<'l, T: 'l>(Option<&'l mut T>);

impl<'l, T: 'l> SpaceIterator<'l, T> {
  pub fn new(init: &'l mut T) -> SpaceIterator<'l, T> {
    SpaceIterator(Some(init))
  }
}

impl<'l, T: Space + 'l> Iterator for SpaceIterator<'l, T> {
  type Item = &'l mut T;
  fn next(&mut self) -> Option<&'l mut T> {
    if let &mut SpaceIterator(Some(space)) = self {
      let coll = space.next_collision();
      let next_space = space.update(coll);
      self.0 = next_space;
    }
    self.0.cloned()
  }
}
*/

