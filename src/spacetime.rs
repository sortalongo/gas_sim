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
      since_last_coll: Time(0.)
    }
  }
}


pub struct SpaceTimeStepIterator<S: Space> {
  spacetime: SpaceTime<S>,
  next_coll: Collision,
  since_last_coll: Time,
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

        trace!("since_last_coll: {:?}", self.since_last_coll);
        let dt_next = coll.t_unsafe();
        trace!("dt_next: {:?}", dt_next);
        let dt_now_next = Time(dt_next.0 - self.since_last_coll.0);
        trace!("dt_now_next: {:?}", dt_now_next);

        let (next_coll, space_next) = if self.step.ge(&dt_now_next) {
          let dt_step = Time(self.step.0 - dt_now_next.0);

          // FIXME: this assumes max 1 collision per timestep
          let space = self.spacetime.space
            // FIXME: this expects the current spacetime to be at
            // the same time as when coll was computed
            .update(&coll).unwrap() // guaranteed update
            .map_particles(|p| p.evolve(dt_step));

          self.since_last_coll = dt_step;

          (space.next_collision(), space)
        } else {
          self.since_last_coll.0 += self.step.0;
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

