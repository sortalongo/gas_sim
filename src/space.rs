use super::{custom_float, FloatOps, Vector,
Particle, Time, Combination2, Combination2Iter};

enum Collision {
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

  fn update(self, collision: Collision) -> Option<Self>;
}

pub struct SpaceVec {
  pub particles: Vec<Particle>,
}

impl<'l> SpaceVec {
  /// Returns an iterator over all pairs of particles
  /// contained in the BoundedBox.
  ///
  /// # Example
  /// ```
  /// # use particles::*;
  ///
  /// let p1 = Particle {
  ///   x: Vector((-2., 0.)),
  ///   v: Vector((1., 0.)),
  ///   r: 1.,
  ///   m: 1.
  /// };
  /// let p2 = Particle {
  ///   x: Vector((2., 0.)),
  ///   v: Vector((-1., 0.)),
  ///   r: 1.,
  ///   m: 1.
  /// };
  /// let pBox = BoundedBoxVec {
  ///   origin: Vector((-5., -5.)),
  ///   corner: Vector((5., 5.)),
  ///   particles: vec![p1, p2],
  /// };
  /// let pairs = pBox.particle_pairs().collect::<Vec<_>>();
  /// println!("{:?}", pairs);
  /// assert!(pairs.len() == 1);
  pub fn particle_pairs(&'l self) -> Combination2Iter<'l, Particle> {
    Combination2(&self.particles).into_iter()
  }
}

impl Space for SpaceVec {
  fn next_collision(&self) -> Collision {
    let pairs = self.particle_pairs();
    let opt_min = pairs.min_by( | pair | {
      let &(p1, p2) = pair;
      if let Some(Time(t)) = p1.impact_time(p2) { FloatOps(t) }
      else { FloatOps(custom_float::INFINITY) }
    });

    if let Some((p1, p2)) = opt_min {
      let t = p1.impact_time(p2).unwrap();
      let (next1, next2) = p1.evolve(t).bounce(&p2.evolve(t));
      Collision::Bounce {
        t: t,
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
        let new_vec: Vec<_> = self.particles.iter().map( |p: &Particle|
          if (p == &prev1) { next1 }
          else if (p == &prev2) { next2 }
          else { p.evolve(t) }
        ).collect();

        Some(SpaceVec { particles: new_vec })
      }
    }
  }
}

