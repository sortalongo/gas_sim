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

  const NUM_PARTICLES: usize = 6;
  const STEP: Time = Time(0.1);

  let max_particle = Particle {
    id: 0,
    x: Vector((5., 5.)),
    v: Vector((1., 1.)),
    m: 1.,
    r: 0.3
  };
  let min_particle = Particle {
    id: usize::MAX,
    x: Vector((-5., -5.)),
    v: Vector((-1., -1.)),
    m: 1.,
    r: 1.0
  };
  let mut rng = StdRng::new().unwrap();

  let init_box = SpaceBox::new_random(&mut rng, NUM_PARTICLES, min_particle, max_particle);
  let init = SpaceTime::new(init_box, Time(0.));

  info!("starting");
  debug!("first state: {:?}", init);

  init.every(STEP)
    .take(100)
    .map(|s| {
      debug!("t: {:?}", s.time);
      if let Some((p1, p2)) = s.space.space_vec().particle_pairs()
        .find(|&(ref p1, ref p2)| p1.overlaps(p2)) {
        warn!("found overlapping particles:\n{:?}\n{:?}", p1, p2);
      }

      let p_str: String = s.space.particles()
        .map(|p| format!("{}\t{}\t{}", (p.x.0).0, (p.x.0).1, p.r))
        .collect::<Vec<_>>()
        .join("\n");
      format!("{}\n\n", p_str)
    })
    .inspect(|s| println!("{}", s))
    .last();

  info!("ending");
}
