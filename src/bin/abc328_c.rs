use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }
    let mut cnt: Vec<usize> = vec![];
    let mut pre = '-';
    let mut c = 0;
    for si in s {
        if si == pre {
            c += 1;
        } else {
            pre = si;
        }
        cnt.push(c);
    }

    for (l, r) in lr.iter() {
        println!("{}", cnt[r-1]-cnt[l-1])
    }
}