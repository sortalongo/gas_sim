use rand::{Rng};
use std::cmp::min;
use std::slice;
use super::{BoundedRand, Bounds, Collision, Particle, Space, SpaceVec, Vector};

#[derive(Debug, Clone)]
pub struct SpaceBox {
  space_vec: SpaceVec,
  bounds: Bounds
}

impl SpaceBox {
  pub fn new(ps: Vec<Particle>, bottom_left: Vector, top_right: Vector) -> SpaceBox {
    let space_vec = SpaceVec::new(ps);
    let bounds = Bounds::new(top_right, bottom_left);
    assert!(
      space_vec.particles().all(|p| bounds.within(p)),
      "bounds must include all particles"
    );
    SpaceBox { space_vec: space_vec, bounds: bounds }
  }

  pub fn new_random<R: Rng>(rng: &mut R, count: usize, min: Particle, max: Particle) -> SpaceBox {
    let mut particles = Vec::with_capacity(count);

    for _ in 0..count {
      let mut new_p: Particle;
      loop {
        new_p = BoundedRand::rand(rng, &min, &max);
        if ! particles.iter().any(|p: &Particle| p.overlaps(&new_p)) { break; }
      }
      particles.push(new_p);
    }

    let r_vec = Vector((max.r, max.r));
    SpaceBox::new(particles, &min.x - &r_vec, &max.x + &r_vec)
  }

  pub fn map_particles<F: FnMut(&Particle) -> Particle>(&self, f: F) -> SpaceBox {
    SpaceBox {
      space_vec: self.space_vec.map_particles(f),
      bounds: self.bounds.clone()
    }
  }
}

impl Space for SpaceBox {

  fn particles(&self) -> slice::Iter<Particle> {
    self.space_vec.particles()
  }

  fn next_collision(&self) -> Collision {
    let inter_particle_coll = self.space_vec.next_collision();
    let wall_coll = self.particles()
      .map(|p| self.bounds.next_collision(p))
      .min()
      .unwrap_or(Collision::Free);

    min(inter_particle_coll, wall_coll)
  }

  fn update(&self, collision: Collision) -> Option<Self> {
    let space_vec_opt = match collision {
      Collision::Wall { t, ref prev, ref next } => {
        let new_vec: Vec<_> = self.particles().map( |p: &Particle|
          if p == prev { next.clone() }
          else { p.evolve(t) }
        ).collect();

        Some(SpaceVec::new(new_vec))
      },
      _ => self.space_vec
        .update(collision)
    };

    space_vec_opt.map(|sv| SpaceBox { space_vec: sv, bounds: self.bounds.clone() } )
  }
}
