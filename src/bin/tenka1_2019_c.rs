use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut w_cnt = Vec::new();
    let mut b_cnt = Vec::new();
    let mut pre_w = 1;
    let mut pre_b = 0;
    w_cnt.push(pre_w);
    b_cnt.push(pre_b);
    for si in s {
        if si == '.' {
            pre_w += 1;
            w_cnt.push(pre_w);
            b_cnt.push(pre_b);
        } else {
            pre_b += 1;
            w_cnt.push(pre_w);
            b_cnt.push(pre_b);
        }
    }

    let mut ans = 200000;
    for i in 0..(n+1) {
        let tmp = b_cnt[i] + w_cnt[n] - w_cnt[i];
        ans = ans.min(tmp);
    }

    println!("{}", ans);
}