use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: [isize; n],
    }
    let mut ans = Vec::new();
    let mut pre = 0;
    for &si in s.iter() {
        ans.push(si - pre);
        pre = si;
    }
    println!("{}", ans.iter().join(" "));
}