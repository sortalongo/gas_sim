extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};

fn main() {
  const NUM_PARTICLES: usize = 10;

  let max_particle = Particle {
    x: Vector((20., 20.)),
    v: Vector((1., 1.)),
    m: 1.,
    r: 1.
  };
  let min_particle = Particle {
    x: Vector((-20., -20.)),
    v: Vector((-1., -1.)),
    m: 1.,
    r: 1.
  };
  let mut rng = StdRng::new().unwrap();

  let mut particles = Vec::with_capacity(NUM_PARTICLES);
  for _ in 0..NUM_PARTICLES {
    let mut new_p: Particle;
    loop {
      new_p = BoundedRand::rand(&mut rng, &min_particle, &max_particle);
      if ! particles.iter().any(|p: &Particle| p.overlaps(&new_p)) { break; }
    }
    particles.push(new_p);
  }

  let mut space = SpaceVec::new(particles);
  let mut i = 0;

  println!("starting: {:?}", space);
  while i < 10 && space.iterate() {
    println!("{}: {:?}", i, space);
    i += 1;
  }
  println!("ending");
}
