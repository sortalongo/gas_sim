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

    for i in 0..count {
      let mut new_p: Particle;
      loop {
        new_p = BoundedRand::rand(rng, &min, &max);
        if ! particles.iter().any(|p: &Particle| p.overlaps(&new_p)) { break; }
      }
      new_p.id = i;
      particles.push(new_p);
    }

    let r_vec = Vector((max.r, max.r));
    SpaceBox::new(particles, &min.x - &r_vec, &max.x + &r_vec)
  }

  pub fn space_vec<'l>(&'l self) -> &'l SpaceVec {
    &self.space_vec
  }
}

impl Space for SpaceBox {

  fn particles(&self) -> slice::Iter<Particle> {
    self.space_vec.particles()
  }

  fn map_particles<F>(&self, f: F) -> SpaceBox
  where F: FnMut(&Particle) -> Particle {
    SpaceBox {
      space_vec: self.space_vec.map_particles(f),
      bounds: self.bounds.clone()
    }
  }

  fn next_collision(&self) -> Collision {
    let inter_particle_coll = self.space_vec.next_collision();
    let wall_coll = self.particles()
      .map(|p| self.bounds.next_collision(p))
      .min()
      .unwrap_or(Collision::Free);

    let first_coll = min(inter_particle_coll, wall_coll);

    debug!("next_collision: {:?}", first_coll);
    if let Collision::Bounce { ref next1, ref next2, .. } = first_coll {
      debug!("bounce distance: {:?}", (&next1.x - &next2.x).norm());
    }

    first_coll
  }

  fn update(&self, collision: &Collision) -> Option<Self> {
    let space_vec_opt = match collision {
      &Collision::Wall { t, ref prev, ref next } => {
        let new_vec: Vec<_> = self.particles().map( |p: &Particle|
          if p.id == prev.id { next.clone() }
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

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::{Collision, Particle, Space, Vector, SpaceVec, Bounds, Time};

  const P1: Particle = Particle {
    id: 0,
    x: Vector((0., 0.)),
    v: Vector((0., 1.)),
    r: 1.,
    m: 1.
  };
  const P2: Particle = Particle {
    id: 0,
    x: Vector((1., 3.)),
    v: Vector((-1., 0.)),
    r: 1.,
    m: 1.
  };
  const TOP_RIGHT: Vector = Vector((5., 5.));
  const BOTTOM_LEFT: Vector = Vector((-5., -5.));

  #[test]
  fn single_particle_hits_wall() {
    let space_box = SpaceBox::new(
      vec![P1], BOTTOM_LEFT, TOP_RIGHT);

    let l = ((TOP_RIGHT.0).1 - P1.r);
    let expected_collision = Collision::Wall {
        t: Time( l / (P1.v.0).1),
        prev: P1,
        next: Particle {
          x: Vector((0., l)),
          v: Vector((0., -1.)),
          .. P1.clone() },
    };
    let collision = space_box.next_collision();
    assert!(
      collision == expected_collision,
      "{:?} did not equal {:?}",
      collision,
      expected_collision);
  }

  #[test]
  fn particles_collide_before_wall() {
    let space_box = SpaceBox::new(
      vec![P1, P2], BOTTOM_LEFT, TOP_RIGHT);

    let l = (P2.x.0).1 - (P1.x.0).1 - P1.r - P2.r;
    let expected_collision = Collision::Bounce {
        t: Time( l / (P1.v.0).1),
        prev1: P1, prev2: P2,
        next1: Particle {
          x: Vector((0., l)),
          v: Vector((0., 0.)),
          .. P1.clone() },
        next2: Particle {
          x: Vector((0., 3.)),
          v: Vector((-1., 1.)),
          .. P2.clone() },
    };
    let collision = space_box.next_collision();
    assert!(
      collision == expected_collision,
      "{:?} did not equal {:?}",
      collision,
      expected_collision);
  }

}
