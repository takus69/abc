use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        u: Chars,
    }
    let n = t.len() - u.len();
    let mut ans = "No";
    for i in 0..=n {
        let mut flg = true;
        for (j, &uj) in u.iter().enumerate() {
            if t[i+j] == '?' { continue; }
            if t[i+j] != uj { flg = false; break; }
        }
        if flg { ans = "Yes"; }
    }
    println!("{}", ans);
}