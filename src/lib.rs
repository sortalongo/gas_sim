pub type CustomFloat = f32;
const SIGNIFICAND: i32 = 24;

#[derive(Debug)]
pub struct Time(pub CustomFloat);

pub use vector::Vector;
pub use particle::{Particle, quadratic_formula};
pub use space::Box;

mod vector;
mod particle;
mod space;

