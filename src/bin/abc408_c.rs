use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    lr: [(usize, usize); m],
  }
  let mut vec: Vec<isize> = vec![0; n+2];
  for &(l, r) in lr.iter() {
    vec[l] += 1;
    vec[r+1] -= 1;
  }
  vec.pop();
  
  let mut sum = 0;
  let mut ans = isize::MAX;
  for &v in vec.iter().skip(1) {
    sum += v;
    ans = ans.min(sum);
  }
  println!("{}", ans);
}