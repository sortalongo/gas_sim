pub struct Combination2<'l, T: 'l>(pub &'l Vec<T>);

impl<'l, T> IntoIterator for Combination2<'l, T> {
  type Item = (&'l T, &'l T);
  type IntoIter = Combination2Iter<'l, T>;

  fn into_iter(self) -> Combination2Iter<'l, T> {
    let Combination2(vec) = self;
    Combination2Iter {
      vec: vec,
      idx1: 0,
      idx2: 1,
    }
  }
}

pub struct Combination2Iter<'l, T: 'l> {
  vec: &'l Vec<T>,
  idx1: usize,
  idx2: usize,
}

impl<'l, T> Iterator for Combination2Iter<'l, T> {
  type Item = (&'l T, &'l T);

  fn next(&mut self) -> Option<(&'l T, &'l T)> {
    let len = self.vec.len();

    if self.idx1 >= len || self.idx2 >= len { None }
    else {
      let item = (&self.vec[self.idx1], &self.vec[self.idx2]);
      if self.idx2 == len - 1 {
        self.idx2 = self.idx1 + 2;
        self.idx1 += 1
      } else { self.idx2 += 1; }
      Some(item)
    }
  }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
  use super::{Combination2};
  use super::quickcheck::quickcheck;

  #[test]
  fn length_is_2_choose_l() {
    fn prop(v: Vec<usize>) -> bool {
      let combo = Combination2(&v).into_iter().collect::<Vec<_>>();
      let n = v.len();

      combo.len() == if n > 0 { n * (n-1) / 2 } else { 0 }
    }
    quickcheck(prop as fn(Vec<usize>)->bool);
  }
}

