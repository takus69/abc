use proconio::input;

fn main() {
  input! {
    n: usize,
    s: usize,
    t: [usize; n],
  }
  let mut pre = 0;
  for &ti in t.iter() {
    if ti-pre > s {
      println!("No");
      return;
    }
    pre = ti;
  }
  println!("Yes");
}