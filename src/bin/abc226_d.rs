use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    
    fn gcd(a: isize, b: isize) -> isize {
        if b==0 { return a; }
        gcd(b, a%b)
    }

    let mut diff: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..(n-1) {
        for j in (i+1)..n {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let dx = xi-xj;
            let dy = yi-yj;
            let g = gcd(dx, dy);
            diff.insert((dx/g, dy/g));
            diff.insert((-dx/g, -dy/g));
        }
    }
    println!("{}", diff.len());
}