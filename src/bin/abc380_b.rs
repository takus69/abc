use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: Vec<usize> = Vec::new();
    for &si in s.iter() {
        if si == '|' {
            ans.push(0);
        } else {
            let i = ans.len()-1;
            ans[i] += 1;
        }
    }
    ans.pop();
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}