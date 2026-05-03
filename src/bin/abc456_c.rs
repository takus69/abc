use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        s: Chars,
    }
    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    let mut pre_si = s[0];
    let mut cnt = 1;
    while l < s.len() {
        // rを進める
        while r < s.len() && s[r] != pre_si {
            pre_si = s[r];
            r += 1;
            cnt += 1;
            cnt %= MOD;
        }
        // println!("r: {}, cnt: {}", r, cnt);
        // lを進める
        while l < r {
            ans += cnt;
            ans %= MOD;
            l += 1;
            cnt += MOD;
            cnt -= 1;
            cnt %= MOD;
        }
        // println!("l: {}, cnt: {}, ans: {}", l, cnt, ans);
        if r < s.len() {
            pre_si = s[r];
            r += 1;
            cnt += 1;
        }
    }

    println!("{}", ans);
}