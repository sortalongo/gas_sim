extern crate fern;
#[macro_use]
extern crate log;
extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};

fn init_logger() {
  fern::init_global_logger(
    fern::OutputConfig::stderr(),
    log::LogLevelFilter::Trace
  ).unwrap();
}

fn main() {
  init_logger();

  const NUM_PARTICLES: usize = 10;
  const STEP: Time = Time(1.);

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
  let init = SpaceTime::new(init_box, Time(0.));

  info!("starting");

  init.every(STEP)
    .enumerate()
    .take(10)
    .inspect(|i_s| debug!("{:?}", i_s))
    .last();

  info!("ending");
}
