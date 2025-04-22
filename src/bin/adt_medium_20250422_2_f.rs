use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    let mut tmp = 0;
    let mut flg = false;
    let mut flg2 = false;
    for &si in s.iter() {
        if si == '-' {
            flg = true;
        }
        if si == 'o' {
            flg2 = true;
            tmp += 1;
        } else {
            ans = ans.max(tmp);
            tmp = 0;
        }
    }
    if flg && flg2 {
        ans = ans.max(tmp);
    }
    if flg && flg2 {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}