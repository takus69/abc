use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: Chars,
    }
    let mut ans: Vec<char> = Vec::new();
    let mut cnt = 0;
    for &si in s.iter().rev() {
        if cnt >= d {
            ans.push(si);
        } else {
            ans.push('.');
            if si == '@' { cnt += 1; }
        }
    }
    ans.reverse();
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}