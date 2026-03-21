use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }

    let mut cnt_a: Vec<usize> = vec![0];
    let mut cnt_b: Vec<usize> = vec![0];
    for &ai in &s {
        if ai == 'a' {
            cnt_a.push(*cnt_a.last().unwrap() + 1);
            cnt_b.push(*cnt_b.last().unwrap());
        } else {
            cnt_a.push(*cnt_a.last().unwrap());
            cnt_b.push(*cnt_b.last().unwrap() + 1);
        }
    }
    cnt_a.push(usize::MAX);
    cnt_b.push(usize::MAX);
    let mut ans = 0;
    for l in 0..=n {
        let la = cnt_a[l];
        let lb = cnt_b[l];
        // println!("cnt_a: {:?}, cnt_b: {:?}", cnt_a, cnt_b);
        let ra = cnt_a.partition_point(|&x| x < la + a);
        let rb = cnt_b.partition_point(|&x| x < lb + b);
        // println!("l: {}, la: {}, lb: {}, ra: {}, rb: {}", l, la, lb, ra, rb);
        if ra < rb {
            ans += rb-ra;
        }
    }

    println!("{}", ans);
}