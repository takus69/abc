use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        c1: char,
        c2: char,
        s: Chars,
    }
    let mut ans: Vec<char> = vec![];
    for &si in s.iter() {
        if si == c1 {
            ans.push(c1);
        } else {
            ans.push(c2);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}