use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }
    let mut before: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
    let (mut t2, mut t1, mut t0) = (0, 0, 0);
    for i in 1..=n {
        let ai = a[i-1];
        t2 += i*i*ai;
        t1 += i*ai;
        t0 += ai;
        before.push((t2, t1, t0));
    }
    for &(l, r) in lr.iter() {
        let (r2, r1, r0) = before[r];
        let (l2, l1, l0) = before[l-1];
        let (i2, i1, i0) = (r2-l2, r1-l1, r0-l0);
        let ans = (l+r)*i1 - (l-1)*(r+1)*i0 - i2;
        println!("{}", ans);
    }
}