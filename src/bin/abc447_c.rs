use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut ans: usize = 0;

    let mut i1 = s.len()-1;
    let mut i2 = t.len()-1;
    while t[i2] == 'A' && s[i1] != 'A' {
        s.push('A');
        ans += 1;
        if i2 == 0 { break; }
        i2 -= 1;
    }

    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < s.len() && i2 < t.len() {
        // println!("i1: {}, i2: {}", i1, i2);
        if s[i1] == 'A' {
            if t[i2] == 'A' {
                i1 += 1;
                i2 += 1;
            } else {
                ans += 1;
                i1 += 1;
            }
        } else {
            if t[i2] == 'A' {
                ans += 1;
                i2 += 1;
            } else {
                if s[i1] == t[i2] {
                    i1 += 1;
                    i2 += 1;
                } else {
                    println!("-1");
                    return;
                }
            }
        }
    }

    if i1 < s.len() {
        for i in i1..s.len() {
            if s[i] != 'A' {
                println!("-1");
                return;
            }
        }
        ans += s.len() - i1;
    } else if i2 < t.len() {
        for i in i2..t.len() {
            if t[i] != 'A' {
                println!("-1");
                return;
            }
        }
        ans += t.len() - i2;
    }

    println!("{}", ans);
}