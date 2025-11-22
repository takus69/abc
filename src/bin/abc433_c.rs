use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut pre_c = -1;
    let mut pre_cnt = 0;
    let mut now_c = -1;
    let mut now_cnt = 0;
    let mut ans = 0;
    for &si in s.iter() {
        let c = si.to_digit(10).unwrap() as i32;
        if now_c == c {
            now_cnt += 1;
        } else {
            pre_c = now_c;
            pre_cnt = now_cnt;
            now_c = c;
            now_cnt = 1;
        }
        if pre_c+1 == now_c && now_cnt <= pre_cnt {
            ans += 1;
        }
    }
    println!("{}", ans);
}