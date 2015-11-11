extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};
use std::mem;

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

  let init_box = SpaceBox::new_random(&mut rng, NUM_PARTICLES, min_particle, max_particle);
  let mut init = SpaceTime::new(init_box, Time(0.));

  println!("starting: {:?}", init);

  let mut space_pairs = init.next().unwrap()
    .scan(init, |prev, mut next| {
       mem::swap(prev, &mut next);
       Some((next, prev.clone()))
    });

  space_pairs
    .enumerate()
    .take(10)
    .inspect(|i_s| println!("{:?}", i_s))
    .last();
  println!("ending");
}
