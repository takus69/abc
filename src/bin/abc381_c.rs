use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 1;
    let mut m: Vec<usize> = Vec::new();
    for (i, &si) in s.iter().enumerate() {
        if si == '/' {
            m.push(i);
        }
    }
    for &mi in m.iter() {
        let mut c1 = 0;
        let mut c2 = 0;
        for i in (mi+1)..n {
            if s[i] == '2' {
                c2 += 1;
            } else {
                break;
            }
        }
        for i in (0..mi).rev() {
            if s[i] == '1' {
                c1 += 1;
            } else {
                break;
            }
        }
        ans = ans.max(1+(c1.min(c2))*2);
    }
    println!("{}", ans);
}