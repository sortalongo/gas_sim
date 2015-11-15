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
      step: step
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

      coll @ Collision::Wall { .. } |
      coll @ Collision::Bounce { .. } => {

        let dt_next_coll = coll.t_unsafe();

        let (next_coll, space_next) = if self.step.ge(&dt_next_coll) {
          let space = self.spacetime.space
            .update(&coll).unwrap() // guaranteed update
            .map_particles(|p| p.evolve(Time(self.step.0 - dt_next_coll.0)));

          (self.spacetime.space.next_collision(), space)
        } else {
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

