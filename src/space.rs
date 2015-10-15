use super::{Vector, Particle};

pub trait Space {
  fn create(initial: Vec<Particle>) -> Self;
  fn next_collision(&self) -> (&Particle, &Particle);
  fn update<'l>(&'l mut self, &p1: Particle, &p2: Particle) -> &'l mut Self;
}

pub struct BoundedBoxVec {
  pub origin: Vector,
  pub corner: Vector,
  pub particles: Vec<Particle>,
}

impl<'l> BoundedBoxVec {
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
  pub fn particle_pairs(&'l self) -> ParticlePairIter<'l> {
    ParticlePairVec(&self.particles).into_iter()
  }

}

struct ParticlePairVec<'l>(&'l Vec<Particle>);

impl<'l> IntoIterator for ParticlePairVec<'l> {
  type Item = (&'l Particle, &'l Particle);
  type IntoIter = ParticlePairIter<'l>;

  fn into_iter(self) -> ParticlePairIter<'l> {
    let ParticlePairVec(particles) = self;
    ParticlePairIter {
      particle_vec: particles,
      idx1: 0,
      idx2: 1,
    }
  }
}

pub struct ParticlePairIter<'l> {
  particle_vec: &'l Vec<Particle>,
  idx1: usize,
  idx2: usize,
}

impl<'l> Iterator for ParticlePairIter<'l> {
  type Item = (&'l Particle, &'l Particle);

  fn next(&mut self) -> Option<(&'l Particle, &'l Particle)> {
    let len = self.particle_vec.len();

    if self.idx1 >= len || self.idx2 >= len { None }
    else {
      let item = (&self.particle_vec[self.idx1], &self.particle_vec[self.idx2]);
      if self.idx2 == len - 1 {
        self.idx2 = self.idx1 + 2;
        self.idx1 += 1
      } else { self.idx2 += 1; }
      Some(item)
    }
  }
}






