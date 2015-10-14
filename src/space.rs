use super::particle::Particle;

pub trait Space {
  fn create(initial: Vec<Particle>) -> Self;
  fn next_collision(&self) -> (&Particle, &Particle);
  fn update<'l>(&'l mut self, &p1: Particle, &p2: Particle) -> &'l mut Self;
}

pub struct BoundedBoxVec {
  pub origin: Vector,
  pub corner: Vector,
  particles: Vec<Particle>
}

