use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut plus: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut minus: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }
        plus[l-1].push(x);
        minus[r].push(x);
    }
    let mut cnt: Vec<usize> = vec![0; q+1];
    let mut set_cnt = 0;
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        for &xi in minus[i].iter() {
            cnt[xi] -= 1;
            if cnt[xi] == 0 {
                set_cnt -= 1;
            }
        }
        for &xi in plus[i].iter() {
            cnt[xi] += 1;
            if cnt[xi] == 1 {
                set_cnt += 1;
            }
        }
        ans.push(set_cnt);
    }
    println!("{}", ans.iter().join(" "));
}