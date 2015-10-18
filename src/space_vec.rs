use super::{Collision, Combination2, Combination2Iter,
  FloatOps, Particle, Space, Time };

pub struct SpaceVec {
  particles: Vec<Particle>,
}

impl<'l> SpaceVec {
  pub fn new(ps: Vec<Particle>) -> SpaceVec {
    SpaceVec { particles: ps }
  }

  // Returns an iterator over all pairs of particles
  // contained in the BoundedBox.
  fn particle_pairs(&'l self) -> Combination2Iter<'l, Particle> {
    Combination2(&self.particles).into_iter()
  }
}

impl Space for SpaceVec {
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
      let (prebounce1, prebounce2) = (p1.evolve(time), p2.evolve(time));
      let (next1, next2) = prebounce1.bounce(&prebounce2);
      Collision::Bounce {
        t: time,
        prev1: p1.clone(), prev2: p2.clone(),
        next1: next1, next2: next2
      }
    } else {
      Collision::Free
    }
  }

  fn update(self, c: Collision) -> Option<Self> {
    match c {
      Collision::Free => None,
      Collision::Wall {..} => unimplemented!(),
      Collision::Bounce { t, prev1, prev2, next1, next2 } => {
        let new_vec: Vec<_> = self.particles.iter().map( move |p: &Particle|
          if p == &prev1 { next1.clone() }
          else if p == &prev2 { next2.clone() }
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
  use super::super::{Particle, Vector};

  const P1: Particle = Particle {
    x: Vector((-2., 0.)),
    v: Vector((1., 0.)),
    r: 1.,
    m: 1.
  };
  const P2: Particle = Particle {
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
    println!("{:?}", pairs);
    assert!(pairs.len() == 1);
  }
}
