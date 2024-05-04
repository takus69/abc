use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut si = 0;
    let mut ti = 0;
    let mut ans = Vec::new();
    while ti < t.len() {
        if s[si] == t[ti] {
            ans.push(ti+1);
            si += 1;
            ti += 1;
        } else {
            ti += 1;
        }
    }
    println!("{}", ans.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(" "));
}