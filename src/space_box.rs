use std::cmp::min;
use std::slice;
use super::{Bounds, Collision, Particle, Space, SpaceVec, Vector };

#[derive(Debug)]
pub struct SpaceBox {
  space_vec: SpaceVec,
  bounds: Bounds
}

impl SpaceBox {
  pub fn new(ps: Vec<Particle>, top_left: Vector, bottom_right: Vector) -> SpaceBox {
    let space_vec = SpaceVec::new(ps);
    let bounds = Bounds::new(top_left, bottom_right);
    assert!(
      space_vec.particles().all(|p| bounds.within(p)),
      "bounds must include all particles"
    );
    SpaceBox { space_vec: space_vec, bounds: bounds }
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

  fn update(&mut self, collision: Collision) -> Option<&mut Self> {
    let space_vec_opt = match collision {
      Collision::Wall { t, ref prev, ref next } => {
        let new_vec: Vec<_> = self.particles().map( |p: &Particle|
          if p == prev { next.clone() }
          else { p.evolve(t) }
        ).collect();

        self.space_vec = SpaceVec::new(new_vec);
        Some(())
      },
      _ => self.space_vec
        .update(collision)
        .map(|_| ()),
    };

    space_vec_opt.map(|_| self )
  }
}
