use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
  }
  let mut cnt = 0;
  for i in 1..=6 {
    for j in 1..=6 {
      if x <= i+j || y <= i.abs_diff(j) {
        cnt += 1;
      }
    }
  }

  println!("{}", cnt as f64 / 36.0);
}