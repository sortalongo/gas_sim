use std::slice;
use super::{Collision, Combination2, Combination2Iter,
  FloatOps, Particle, Space, Time };

#[derive(Debug, Clone)]
pub struct SpaceVec {
  particles: Vec<Particle>,
}

impl<'l> SpaceVec {
  pub fn new(ps: Vec<Particle>) -> SpaceVec {
    let space_vec = SpaceVec { particles: ps };
    assert!(
      !space_vec.particle_pairs().any(|pair| (pair.0).overlaps(&pair.1)),
      "SpaceVec initialized with overlapping particles"
    );
    space_vec
  }

  // Returns an iterator over all pairs of particles
  // contained in the Space.
  pub fn particle_pairs(&'l self) -> Combination2Iter<'l, Particle> {
    Combination2(&self.particles).into_iter()
  }
}

impl Space for SpaceVec {
  fn particles(&self) -> slice::Iter<Particle> {
    self.particles.iter()
  }

  fn map_particles<F>(&self, f: F) -> SpaceVec
  where F: FnMut(&Particle) -> Particle {
    SpaceVec {
      particles: self.particles.iter()
        .map(f)
        .collect()
    }
  }

  fn next_collision(&self) -> Collision {
    let pairs = self.particle_pairs();
    let opt_min = pairs.fold(None, | opt_min, pair | {
      let (p1, p2) = pair;
      match (opt_min, p1.impact_time(p2)) {
        (Some((curr_min, min_pair)), Some(Time(t))) =>
          if FloatOps(t) < curr_min {
            Some((FloatOps(t), pair))
          } else {
            // have to rewrite the incoming pattern
            // due to missing feature in match borrow checker
            Some((curr_min, min_pair))
          },

        (None, Some(Time(t))) =>
          Some((FloatOps(t), pair)),

        _ => opt_min
      }
    });

    if let Some((FloatOps(t), (p1, p2))) = opt_min {
      let time = Time(t);
      let (next1, next2) = p1.after_bounce(&p2, time);
      Collision::Bounce {
        t: time,
        prev1: p1.clone(), prev2: p2.clone(),
        next1: next1, next2: next2
      }
    } else {
      Collision::Free
    }
  }

  fn update(&self, c: &Collision) -> Option<Self> {
    match c {
      &Collision::Free => None,
      &Collision::Wall {..} => unreachable!(),
      &Collision::Bounce { t, ref prev1, ref prev2, ref next1, ref next2 } => {
        let new_vec: Vec<_> = self.particles.iter().map( move |p: &Particle|
          if p.id == prev1.id { next1.clone() }
          else if p.id == prev2.id { next2.clone() }
          else { p.evolve(t) }
        ).collect();

        Some(SpaceVec { particles: new_vec })
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::{Particle, Space, Vector};

  const P1: Particle = Particle {
    id: 1,
    x: Vector((-2., 0.)),
    v: Vector((1., 0.)),
    r: 1.,
    m: 1.
  };
  const P2: Particle = Particle {
    id: 2,
    x: Vector((2., 0.)),
    v: Vector((-1., 0.)),
    r: 1.,
    m: 1.
  };

  #[test]
  fn two_particles_yield_one_pair() {
    let p_box = SpaceVec {
      particles: vec![P1, P2],
    };
    let pairs = p_box.particle_pairs().collect::<Vec<_>>();
    assert!(pairs.len() == 1);
  }
}
