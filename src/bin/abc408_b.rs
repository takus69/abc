use proconio::input;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let set: HashSet<usize> = a.into_iter().collect();
  let mut vec: Vec<usize> = set.into_iter().collect();
  vec.sort();
  println!("{}", vec.len());
  println!("{}", vec.iter().join(" "));
}