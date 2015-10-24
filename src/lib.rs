extern crate rand;

pub type CustomFloat = f32;
mod custom_float {
  pub use std::f32::*;
}

#[derive(Debug, Clone, Copy)]
pub struct Time(pub CustomFloat);

pub use float::FloatOps;
pub use vector::Vector;
pub use particle::{Particle};
pub use bounded_rand::BoundedRand;
pub use cartesian_iter::{Combination2, Combination2Iter};
pub use space::{Collision, Space};
pub use space_vec::SpaceVec;

mod float;
mod vector;
mod particle;
mod bounded_rand;
mod cartesian_iter;
mod space;
mod space_vec;

