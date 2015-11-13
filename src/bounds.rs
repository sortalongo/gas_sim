use super::{Collision, FloatOps, Particle, Time, Vector};
use std::cmp::{min, max};

#[derive(Debug, Clone)]
pub struct Bounds {
  top_right: Vector,
  bottom_left: Vector
}

impl Bounds {
  pub fn new(top_right: Vector, bottom_left: Vector) -> Bounds {
    Bounds::check_bounds(&top_right, &bottom_left);
    Bounds { top_right: top_right, bottom_left: bottom_left }
  }

  fn check_bounds<'l>(top_right: &'l Vector, bottom_left: &'l Vector) {
    let Vector((dx, dy)) = top_right - bottom_left;
    assert!(FloatOps(dx) > FloatOps(0.),
      "top left of box must be left of bottom right");
    assert!(FloatOps(dy) > FloatOps(0.),
      "top left of box must be above bottom right");
  }

  pub fn within(&self, p: &Particle) -> bool {
    let Vector((right, top)) = self.top_right;
    let Vector((left, bottom)) = self.bottom_left;
    let Vector((x, y)) = p.x;

    FloatOps(left) <= FloatOps(x) &&
    FloatOps(x) <= FloatOps(right) &&
    FloatOps(bottom) <= FloatOps(y) &&
    FloatOps(y) <= FloatOps(top)
  }

  pub fn next_collision(&self, p: &Particle) -> Collision {
    let Vector((mut vx, mut vy)) = p.v;

    let Vector((xx, xy)) = p.x;
    let Vector((rx, ty)) = self.top_right;
    let Vector((lx, by)) = self.bottom_left;

    let dx = if FloatOps(vx) >= FloatOps(0.) {
      vx *= -1.;
      (xx - rx).abs()
    } else {
      (xx - lx).abs()
    } - p.r;

    let dy = if FloatOps(vy) >= FloatOps(0.) {
      vy *= -1.;
      (xy - by).abs()
    } else {
      (xy - ty).abs()
    } - p.r;

    let t = min(
      FloatOps((dx / vx).abs()),
      FloatOps((dy / vy).abs())
    );

    if t <= FloatOps(0.) { Collision::Free } else {
      let time = Time(t.0);
      let p_next = {
        let p_ev = p.evolve(time);
        Particle { v: Vector((vx, vy)), .. p_ev }
      };
      Collision::Wall {
        t: time,
        prev: p.clone(),
        next: p_next
      }
    }
  }
}

