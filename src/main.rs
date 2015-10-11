extern crate gnuplot;

use std::io;
use gnuplot::{Axes2D, AxesCommon, Caption, Color,
  Figure, Fix, PointSymbol, PointSize};

fn plot(axes: &mut Axes2D, x: &[f64], y: &[f64]) {
  axes
    .set_x_range(Fix(-5.), Fix(5.))
    .set_y_range(Fix(-5.), Fix(5.))
    .points(x, y, &[
      Color("blue"),
      PointSymbol('O'),
      PointSize(5.)
  ]);
}

fn main() {
  let file = "output.gif";

  let mut fg = Figure::new();
  fg.set_terminal(&"gif animate delay 50 loop 0", &file);

  let mut x = [0f64, 1., 2.];
  let mut y = [2f64, 3., 4.];

  plot(fg.axes2d(), &x, &y);


  x[0] -= 1.;
  y[2] -= 3.;
  plot(fg.axes2d(), &x, &y);

  y[0] -= 1.;
  y[2] -= 3.;
  plot(fg.axes2d(), &x, &y);
  fg.echo(&mut io::stdout());
  fg.show();

  println!("image saved");
}
