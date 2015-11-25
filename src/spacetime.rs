use std::mem;
use super::{Collision, Space, Time};


#[derive(Debug, Clone)]
pub struct SpaceTime<S: Space> {
  pub space: S,
  pub time: Time
}

impl<S: Space> SpaceTime<S> {
  pub fn new(s: S, t: Time) -> SpaceTime<S> {
    SpaceTime {
      space: s,
      time: t
    }
  }

  pub fn every(self, step: Time) -> SpaceTimeStepIterator<S> {
    SpaceTimeStepIterator {
      next_coll: self.space.next_collision(),
      spacetime: self,
      step: step,
    }
  }
}


pub struct SpaceTimeStepIterator<S: Space> {
  spacetime: SpaceTime<S>,
  next_coll: Collision,
  step: Time
}

impl<S: Space + Clone> Iterator for SpaceTimeStepIterator<S> {
  type Item = SpaceTime<S>;

  fn next(&mut self) -> Option<SpaceTime<S>> {
    let t_0 = self.spacetime.time;
    let t_next = Time(t_0.0 + self.step.0);

    match self.next_coll.clone() {
      Collision::Free => None,

      mut coll @ Collision::Wall { .. } |
      mut coll @ Collision::Bounce { .. } => {
        let (next_coll, space_next) = if self.step.ge(&coll.t_cpy()) {
          let dt_step = Time(self.step.0 - coll.t_cpy().0);

          // FIXME: this assumes max 1 collision per timestep
          let space = self.spacetime.space
            .update(&coll).unwrap() // guaranteed update
            .map_particles(|p| p.evolve(dt_step));

          coll.t_mut().0 -= dt_step.0;

          (space.next_collision(), space)
        } else {
          coll.t_mut().0 -= self.step.0;
          (coll,
           self.spacetime.space.map_particles(|p| p.evolve(self.step))
          )
        };

        self.next_coll = next_coll;

        let mut spacetime_next = SpaceTime::new(space_next, t_next);
        mem::swap(&mut self.spacetime, &mut spacetime_next);

        Some(spacetime_next)
      }
    }
  }
}

