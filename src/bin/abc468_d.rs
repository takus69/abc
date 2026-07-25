use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    let n = s.len();
    for i in 0..n {
        // 偶数の場合
        if i < n-1 {
            let mut cnt = 0;
            for j in 0..=i.min(n-i-2) {
                if s[i-j] != s[i+j+1] {
                    cnt += 1;
                }
                if cnt <= 1 {
                    ans += 1;
                } else {
                    break;
                }
            }
        }

        // 奇数の場合
        let mut cnt = 0;
        for j in 0..=i.min(n-i-1) {
            if s[i-j] != s[i+j] {
                cnt += 1;
            }
            if cnt <= 1 {
                ans += 1;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}