use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }
    let mut ki: usize = 0;  // 何番目の1の塊か
    let mut ki1: usize = 0;  // k-1番目のインデックス
    let mut ki2: usize = 0;  // k番目のインデックス
    let mut n1: usize = 0;  // k-1番目の個数
    let mut n2: usize = 0;  // k番目の個数
    let mut pre: char = '0';
    s.push('0');
    for (i, &si) in s.iter().enumerate() {
        if si != pre {
            pre = si;
            if si == '1' {
                ki += 1;
                if ki == k-1 {
                    ki1 = i;
                } else if ki == k {
                    ki2 = i;
                }
            } else {
                if ki == k-1 {
                    n1 = i - ki1;
                } else if ki == k {
                    n2 = i - ki2;
                }
            }
        }
    }
    let mut ans: Vec<String> = Vec::new();
    for i in 0..(ki1+n1) {
        ans.push(s[i].to_string());
    }
    for i in ki2..(ki2+n2) {
        ans.push(s[i].to_string());
    }
    for i in (ki1+n1)..ki2 {
        ans.push(s[i].to_string());
    }
    for i in (ki2+n2)..n {
        ans.push(s[i].to_string());
    }
    println!("{}", ans.join(""));
}