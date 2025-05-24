use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
  }
  let mut ans = 0;
  let mut diff = usize::MAX;
  for i in 0..=407 {
    let diff2 = a.abs_diff(b*i);
    if diff > diff2 {
      ans = i;
      diff = diff2;
    }
  }
  println!("{}", ans);
}