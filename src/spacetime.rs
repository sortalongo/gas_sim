use std::mem;
use super::{Collision, Space, Time};


#[derive(Debug, Clone)]
pub struct SpaceTime<S: Space> {
  pub space: S,
  pub time: Time
}

impl<S: Space + Clone> SpaceTime<S> {
  pub fn new(s: S, t: Time) -> SpaceTime<S> {
    SpaceTime {
      space: s,
      time: t
    }
  }

  fn advance(&self, dt: Time) -> SpaceTime<S> {
    SpaceTime::new(
      self.space.map_particles(|p| p.evolve(dt)),
      Time(self.time.0 + dt.0)
    )
  }

  fn update(&self, coll: &Collision) -> SpaceTime<S> {
    match coll {
      &Collision::Free => self.clone(),

      &Collision::Wall { t, .. } |
      &Collision::Bounce { t, .. } =>
        SpaceTime::new(
          self.space.update(coll)
          .expect(&format!("SpaceTime::update unable to update child space")),
          Time(self.time.0 + t.0)
        )
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
    match self.next_coll {
      Collision::Free => {
        let mut spacetime_next = self.spacetime.advance(self.step);
        mem::swap(&mut self.spacetime, &mut spacetime_next);
        Some(spacetime_next)
      },

      Collision::Wall { .. } |
      Collision::Bounce { .. } => {
        let to_return = self.spacetime.clone();

        let mut dt_step = self.step;
        let mut dt_coll = self.next_coll.t();

        while dt_coll.lt(&dt_step) {
          let spacetime_next = self.spacetime.update(&self.next_coll);
          dt_step.0 -= dt_coll.0;

          self.next_coll = spacetime_next.space.next_collision();
          dt_coll = self.next_coll.t();

          self.spacetime = spacetime_next;
        }
        self.spacetime = self.spacetime.advance(dt_step);
        self.next_coll.t_mut().0 -= dt_step.0;

        Some(to_return)
      }
    }
  }
}

