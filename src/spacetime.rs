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

  /*
  type EverySpaceTimeIter<S> = iter::Scan<S, S, >;
  pub fn every(&mut self, t: Time) -> SpaceTime<S> {
    let mut space_pairs = self
      .scan(init, |prev, mut next| {
         mem::swap(prev, &mut next);
         Some((next, prev.clone()))
      });
  }
  */
}

impl<S: Space> Iterator for SpaceTime<S> {
  type Item = SpaceTime<S>;
  fn next(&mut self) -> Option<SpaceTime<S>> {
    let Time(t0) = self.time;
    trace!("time: {}", t0);

    match self.space.next_collision() {
      Collision::Free => None,

      c @ Collision::Wall { .. } |
      c @ Collision::Bounce { .. } => {

        let Time(t) = match c { // annoying limitation in pattern matching
          Collision::Free => unreachable!(),
          Collision::Wall { t, .. } |
          Collision::Bounce { t, .. } => t
        };

        debug!("Next collision: {:?}", &c);

        self.space.update(c).map(|new_space| {
          let mut new_spacetime = SpaceTime::new(new_space, Time(t0 + t));
          mem::swap(&mut new_spacetime, self);
          new_spacetime
        })
      }
    }
  }
}

