use std::println;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let mut left: Vec<isize> = Vec::new();
    let mut right: Vec<isize> = Vec::new();
    for &ai in &a {
        if ai < 0 {
            left.push(ai);
        } else {
            right.push(ai);
        }
    }
    left.sort();
    right.sort();right.reverse();
    // println!("left: {:?}, right: {:?}", left, right);
    const MIN: isize = -10_000_000_000;
    const MAX: isize = 10_000_000_000;
    let mut ans = 0;
    let mut now = 0;
    let mut l = MIN;
    let mut r = MAX;
    loop {
        if l == MIN && !left.is_empty() {
            l = left.pop().unwrap();
        }
        if r == MAX && !right.is_empty() {
            r = right.pop().unwrap();
        }
        if l==MIN && r==MAX { break; }
        // println!("ans: {}, now: {}, l: {}, r: {}", ans, now, l, r);
        if r - now >= now - l {
            ans += now - l;
            now = l;
            l = MIN;
        } else {
            ans += r - now;
            now = r;
            r = MAX;
        }
    }
    println!("{}", ans);
}