use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let n = a.len();
    let mut ans = 0;
    let mut i = 0;
    let mut j = 1;
    while j < n {
        if a[i] <= a[j]/2 {
            ans += n-j;
            i += 1;
        } else {
            j += 1;
        }
    }

    println!("{}", ans);
}