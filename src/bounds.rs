use super::{Collision, FloatOps, Particle, Time, Vector};

#[derive(Debug, Clone)]
pub struct Bounds {
  top_left: Vector,
  bottom_right: Vector
}

impl Bounds {
  pub fn new(top_left: Vector, bottom_right: Vector) -> Bounds {
    Bounds::check_bounds(&top_left, &bottom_right);
    Bounds { top_left: top_left, bottom_right: bottom_right }
  }

  fn check_bounds<'l>(top_left: &'l Vector, bottom_right: &'l Vector) {
    let Vector((dx, dy)) = top_left - bottom_right;
    assert!(FloatOps(dx) < FloatOps(0.),
      "top left of box must be left of bottom right");
    assert!(FloatOps(dy) > FloatOps(0.),
      "top left of box must be above bottom right");
  }

  pub fn within(&self, p: &Particle) -> bool {
    let Vector((left, top)) = self.top_left;
    let Vector((right, bottom)) = self.bottom_right;
    let Vector((x, y)) = p.x;

    FloatOps(left) <= FloatOps(x) &&
    FloatOps(x) <= FloatOps(right) &&
    FloatOps(bottom) <= FloatOps(y) &&
    FloatOps(y) <= FloatOps(top)
  }

  pub fn next_collision(&self, p: &Particle) -> Collision {
    let Vector((mut vx, mut vy)) = p.v;

    let Vector((xx, xy)) = p.x;
    let Vector((lx, ty)) = self.top_left;
    let Vector((rx, by)) = self.bottom_right;

    let dx = if FloatOps(vx) >= FloatOps(0.) {
      vx *= -1.;
      (xx - rx).abs()
    } else {
      (xx - lx).abs()
    };
    let dy = if FloatOps(vy) >= FloatOps(0.) {
      vy *= -1.;
      (xy - by).abs()
    } else {
      (xy - ty).abs()
    };

    let t = Time(((dx / vx).abs()).min((dy / vy).abs()));
    let p_next = {
      let p_ev = p.evolve(t);
      Particle { v: Vector((vx, vy)), .. p_ev }
    };
    Collision::Wall {
      t: t,
      prev: p.clone(),
      next: p_next
    }
  }
}

