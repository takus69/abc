use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = "No";
    for w in 1..s.len() {
        for c in 1..=w {
            // println!("(w, c)=({}, {})", w, c);
            let mut tmp = Vec::new();
            let mut flg = true;
            for i in 0..(s.len()/w+1) {
                // println!("{}", w*i+c-1);
                if w*i+c-1 < s.len() {
                    tmp.push(s[w*i+c-1]);
                }
            }
            // println!("tmp: {:?}", tmp);
            if t.len() != tmp.len() {
                continue;
            }
            for i in 0..t.len() {
                if t[i] != tmp[i] {
                    flg = false;
                    break;
                }
            }
            if flg {
                ans = "Yes";
                break;
            }
        }
    }
    println!("{}", ans);
}