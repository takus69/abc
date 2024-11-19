use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize; q],
    }
    let mut ans: Vec<String> = Vec::new();
    let n = s.len();
    for &ki in k.iter() {
        let mut ni = n;
        while ki > ni {
            ni *= 2;
        }
        let mut ki = ki;
        let mut i = 0;
        while ki > n {
            if ki > ni {
                ki -= ni;
                i += 1;
            }
            ni /= 2;
        }
        let j = (ki-1) % n;
        let mut si = s[j];
        if i % 2 == 1 {
            if si.is_lowercase() {
                si = si.to_ascii_uppercase();
            } else {
                si = si.to_ascii_lowercase();
            }
        }
        ans.push(si.to_string());
    }
    println!("{}", ans.join(" "));
}