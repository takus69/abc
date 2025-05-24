use proconio::input;
use std::collections::BinaryHeap;

fn main() {
  input! {
    t: usize,
  }
  for _ in 0..t {
    input! {
      n: usize,
      a: [usize; 2*n],
    }
    
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    let mut ans = a[0];
    for i in 1..n {
      heap.push(a[2*i-1]);
      heap.push(a[2*i]);

      ans += heap.pop().unwrap();
    }
    println!("{}", ans);
  }
}