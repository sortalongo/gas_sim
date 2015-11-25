use super::{Collision, FloatOps, Particle, Time, Vector, custom_float};

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

    let dx = if vx.ge(&0.) { (xx - rx).abs() }
      else { (xx - lx).abs() } - p.r;

    let dy = if vy.ge(&0.) { (xy - by).abs() }
      else { (xy - ty).abs() } - p.r;

    let dxt = (dx / vx).abs();
    let dyt = (dy / vy).abs();
    let t = if dxt.le(&dyt) {
      vx *= -1.;
      dxt
    } else if dyt.le(&dxt) { // protect against INF
      vy *= -1.;
      dyt
    } else {
      custom_float::INFINITY
    };

    print!("dx: {}, dy: {}, dxt: {}, dyt: {}, t: {}, t.le(0.): {}",
      dx, dy, dxt, dyt, t, t.le(&0.));

    if t.le(&0.) || !t.is_finite() {
      error!("Bounds encountered an illegal state: t: {}, p: {:?}", t, p);
      Collision::Free
    } else {
      let time = Time(t);
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

