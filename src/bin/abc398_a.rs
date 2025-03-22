use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let mut ans: Vec<char> = vec!['-'; (n-1)/2];
    if n%2 == 0 {
        ans.push('=');
        ans.push('=');
    } else {
        ans.push('=');
    }
    for _ in 0..(n-1)/2 {
        ans.push('-');
    }
    println!("{}", ans.iter().join(""));
}