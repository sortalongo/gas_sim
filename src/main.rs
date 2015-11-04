extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};

fn main() {
  const NUM_PARTICLES: usize = 10;

  let top_left = Vector((-20., 20.));
  let bottom_right = Vector((20., -20.));

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

  let space = SpaceBox::new(particles, top_left, bottom_right);

  println!("starting: {:?}", space);
  SpaceIterator::new(space)
    .enumerate()
    .take(10)
    .inspect(|i_s| println!("{:?}", i_s))
    .last();
  println!("ending");
}
