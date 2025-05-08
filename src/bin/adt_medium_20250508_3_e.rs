use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut rk_ = 0;
    let mut lk = 0;
    let mut rk = 0;
    let mut cnt = 0;
    let mut pre = '0';
    for (i, &si) in s.iter().enumerate() {
        if pre == '0' && si != pre {
            cnt += 1;
        }
        if cnt == k-1 && pre == '1' && si != pre {
            rk_ = i-1;
        }
        if cnt == k && pre == '0' && si != pre {
            lk = i;
        }
        if cnt == k && pre == '1' && si != pre {
            rk = i;
        }
        pre = si;
    }
    if rk == 0 {
        rk = n;
    }
    let k_cnt = rk - lk + 1;
    let mut ans: Vec<char> = Vec::new();
    for (i, &si) in s.iter().enumerate() {
        if rk_ <= i && i < rk_ + k_cnt {
            ans.push('1');
        } else if rk_ + k_cnt <= i && i <= rk {
            ans.push('0');
        } else {
            ans.push(si);
        }
    }

    println!("{}", ans.iter().join(""));
}