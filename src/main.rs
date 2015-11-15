extern crate fern;
#[macro_use]
extern crate log;
extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};
use std::mem;
use std::cmp::max;

fn init_logger() {
  fern::init_global_logger(
    fern::OutputConfig::stderr(),
    log::LogLevelFilter::Trace
  ).unwrap();
}

fn main() {
  init_logger();

  const NUM_PARTICLES: usize = 10;
  const STEP: CustomFloat = 1.;

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

  info!("starting");

  fn ceil(f: CustomFloat, step: CustomFloat) -> CustomFloat {
    let rem = f % step;
    f - rem + step
  }
  fn floor(f: CustomFloat, step: CustomFloat) -> CustomFloat {
    let rem = f % step;
    f - rem
  }

  let mut space_pairs = init.next().unwrap()
    .scan(init, |prev, mut next| {
       mem::swap(prev, &mut next);
       Some((next, prev.clone()))
    })
    .flat_map(|(prev, next)| {
      let t_start = ceil(prev.time.0, STEP);
      let t_end = ceil(next.time.0, STEP);
      trace!("start: {}, end: {}", t_start, t_end);

      let steps = max(0, ((t_end - t_start) / STEP).trunc() as i32);
      trace!("steps: {}", steps);
      (0 .. steps).map(move |i| {
        let t = Time(t_start + (i as CustomFloat) * STEP);
        // evolve prev to t, return
        let s = prev.space.map_particles(|p| p.evolve(t));
        SpaceTime::new(s, t)
      })
    });

  space_pairs
    .enumerate()
    .take(10)
    .inspect(|i_s| debug!("{:?}", i_s))
    .last();

  info!("ending");
}
