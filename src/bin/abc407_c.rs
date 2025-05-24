use proconio::{input, marker::Chars};

fn main() {
  input! {
    s: Chars,
  }
  let mut ans = 0;
  for si in s.iter().rev() {
    let mut d = si.to_digit(10).unwrap() as usize;
    if d >= ans%10 {
      d -= ans%10;
    } else {
      d += 10;
      d -= ans%10;
    }
    ans += d;
  }

  ans += s.len();
  println!("{}", ans);
}