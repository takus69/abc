use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }
    for i in 0..(n-1) {
        if s[i] == 'o' && s[i+1] == '?' {
            s[i+1] = '.';
        }
        if s[i] == '?' && s[i+1] == 'o' {
            s[i] = '.';
        }
    }
    let mut cnt: Vec<usize> = Vec::new();
    let mut cnt2: Vec<usize> = Vec::new();
    let mut tmp = 0;
    let mut o_cnt = 0;
    for &si in s.iter() {
        if si == 'o' { o_cnt += 1; }
        if si == '?' {
            tmp += 1;
        } else {
            if tmp > 0 {
                cnt.extend(vec![tmp; tmp]);
            }
            cnt2.push(tmp);
            cnt.push(0);
            tmp = 0;
        }
    }
    if tmp > 0 {
        cnt.extend(vec![tmp; tmp]);
        cnt2.push(tmp);
    }
    let mut max = 0;
    for &c in cnt2.iter() {
        if c == 0 { continue; }
        max += (c+1)/2;
    }
    if o_cnt + max == k {
        let mut j = 0;
        for i in 0..n {
            let si = s[i];
            if cnt[i] == 0 {
                j = 0;
            } else {
                j += 1;
            }
            if cnt[i] % 2 == 1 {
                if j % 2 == 1 {
                    s[i] = 'o';
                } else {
                    s[i] = '.';
                }
            }
        }
    }
    let mut ans: Vec<char> = Vec::new();
    for i in 0..n {
        let si = s[i];
        if si != '?' {
            ans.push(si);
        } else {
            if o_cnt == k {
                ans.push('.');
            } else if o_cnt + max == k {
                if cnt[i] > 1 {
                    ans.push('?');
                } else {
                    ans.push('o');
                }
            } else {
                ans.push('?');
            }
        }
    }
    println!("{}", ans.iter().join(""));
}