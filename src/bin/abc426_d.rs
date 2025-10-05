use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        let mut max0 = 0;
        let mut max1 = 0;
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        let mut tmp = 0;
        let mut now = '-';
        for &si in s.iter() {
            if si == '0' {
                cnt0 += 1;
            } else {
                cnt1 += 1;
            }
            if now == si {
                tmp += 1;
            } else {
                tmp = 1;
                now = si;
            }
            if now == '0' {
                max0 = max0.max(tmp);
            } else {
                max1 = max1.max(tmp);
            }
        }
        let mut ans = 2*(cnt0-max0) + cnt1;
        ans = ans.min(2*(cnt1-max1)+cnt0);
        if cnt0 == 0 || cnt1 == 0 { ans = 0 };
        // println!("max0: {}, max1: {}, cnt0: {}, cnt1: {}", max0, max1, cnt0, cnt1);
        println!("{}", ans);
    }
}