use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        x: usize,
    }
    let mut ans: Vec<char> = Vec::new();
    for (i, &si) in "HelloWorld".chars().collect::<Vec<char>>().iter().enumerate() {
        if i == x-1 { continue; }
        ans.push(si);
    }

    println!("{}", ans.iter().join(""));
}