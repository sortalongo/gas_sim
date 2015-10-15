use super::{Vector, Particle, CartesianProduct, CartesianProductIter};

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
  pub fn particle_pairs(&'l self) -> CartesianProductIter<'l, Particle> {
    CartesianProduct(&self.particles).into_iter()
  }
}

