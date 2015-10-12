pub type CustomFloat = f32;
const SIGNIFICAND: i32 = 24;

#[derive(Debug)]
pub struct Time(pub CustomFloat);

pub use vector::Vector;
pub use particle::Particle;
pub use space::Box;

mod vector;
pub mod particle;
mod space;

