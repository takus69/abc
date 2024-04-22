use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
    }

    let mut chet: Vec<(i64, i64)> = Vec::new();
    let mut b_l = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        if c == &'(' {
            b_l.push_back(i as i64);
        } else if c == &')' {
            let r = i as i64;
            let l = b_l.pop_back().unwrap();
            chet.push((l, r));
        }
    }
    let mut bra = chet.clone();
    bra.sort();

    let mut ans: Vec<(i64, char)> = Vec::new();
    let mut bra_i= 0;
    let mut chet_i= 0;
    let mut p: i64 = 0;
    let mut p_flg: i64 = 1;
    for (i, c) in s.iter().enumerate() {
        if c == &'(' {
            bra_i += 1;
            p_flg *= -1;
            let (l, r) = bra[bra_i-1];
            p -= p_flg*(l + r);
            // println!("i: {}, p: {}", i, p);
        } else if c == &')' {
            chet_i += 1;
            p_flg *= -1;
            let (l, r) = chet[chet_i-1];
            p -= p_flg*(l + r);
        } else {
            let k = p + (p_flg * i as i64);
            let mut s = *c;
            if (bra_i - chet_i)% 2 == 1 {
                if c.is_uppercase() { s = s.to_ascii_lowercase(); }
                else { s = s.to_ascii_uppercase(); }
            }
            ans.push((k, s));
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    let mut a = String::from("");
    for (_, s) in &ans {
        a += &s.to_string();
    }
    // println!("{:?}", ans);
    println!("{}", a);
}