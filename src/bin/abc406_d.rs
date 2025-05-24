  use proconio::input;
  use std::collections::BTreeSet;

  fn main() {
    input! {
      h: usize,
      w: usize,
      n: usize,
      xy: [(usize, usize); n],
      q: usize,
    }
    let mut x_set: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); h];
    let mut y_set: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); w];
    for &(x, y) in xy.iter() {
      let x = x-1;
      let y = y-1;
      x_set[x].insert(y);
      y_set[y].insert(x);
    }
    
    let mut ans: Vec<usize> = Vec::new();
    for _ in 0..q {
      input! {
        c: usize,
      }
      match c {
        1 => {
          input! { x:usize }
          let x = x-1;
          ans.push(x_set[x].len());
          for &y in x_set[x].iter() {
            y_set[y].remove(&x);
          }
          x_set[x] = BTreeSet::new();
        },
        2 => {
          input! { y: usize }
          let y = y-1;
          ans.push(y_set[y].len());
          for &x in y_set[y].iter() {
            x_set[x].remove(&y);
          }
          y_set[y] = BTreeSet::new();
        },
        _ => {},
      }
    }
    
    for a in ans.iter() {
      println!("{}", a);
    }
}