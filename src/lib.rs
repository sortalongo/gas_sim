pub type CustomFloat = f32;
mod custom_float {
  pub use std::f32::*;
}

#[derive(Debug, Clone, Copy)]
pub struct Time(pub CustomFloat);

pub use float::FloatOps;
pub use vector::Vector;
pub use particle::{Particle, quadratic_formula};
pub use space::SpaceVec;
pub use cartesian_iter::{Combination2, Combination2Iter};

mod float;
mod vector;
mod particle;
mod space;
mod cartesian_iter;

