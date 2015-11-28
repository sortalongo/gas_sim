extern crate fern;
#[macro_use]
extern crate log;
extern crate particles;
extern crate rand;

use std::usize;
use std::string::String;
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

  const NUM_PARTICLES: usize = 3;
  const STEP: Time = Time(1.);

  let max_particle = Particle {
    id: 0,
    x: Vector((5., 5.)),
    v: Vector((1., 1.)),
    m: 1.,
    r: 1.
  };
  let min_particle = Particle {
    id: usize::MAX,
    x: Vector((-5., -5.)),
    v: Vector((-1., -1.)),
    m: 1.,
    r: 1.
  };
  let mut rng = StdRng::new().unwrap();

  let init_box = SpaceBox::new_random(&mut rng, NUM_PARTICLES, min_particle, max_particle);
  let init = SpaceTime::new(init_box, Time(0.));

  info!("starting");
  debug!("first state: {:?}", init);

  init.every(STEP)
    .take(10)
    .map(|s| {
      debug!("t: {:?}", s.time);
      let p_str: String = s.space.particles()
        .map(|p| format!("{}\t{}", (p.x.0).0, (p.x.0).1))
        .collect::<Vec<_>>()
        .join("\n");
      format!("{}\n\n", p_str)
    })
    .inspect(|s| println!("{}", s))
    .last();

  info!("ending");
}
