pub type CustomFloat = f32;
const SIGNIFICAND: i32 = 24;

#[derive(Debug)]
pub struct Time(pub CustomFloat);

pub use vector::Vector;
pub use particle::{Particle, quadratic_formula};
pub use space::BoundedBoxVec;
pub use cartesian_iter::{CartesianProduct, CartesianProductIter};

mod vector;
mod particle;
mod space;
mod cartesian_iter;

