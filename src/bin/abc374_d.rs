use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
        abcd: [(i64, i64, i64, i64); n],
    }

    let mut ans: f64 = f64::MAX;
    for perm in (0..n).permutations(n) {
        for i in 0..1<<n {
            let mut time = 0.0;
            let mut now = (0, 0);
            for j in 0..n {
                let (mut sx, mut sy, mut tx, mut ty) = abcd[perm[j]];
                if i>>j & 1 == 1 {
                    std::mem::swap(&mut sx, &mut tx);
                    std::mem::swap(&mut sy, &mut ty);
                }
                time += (((now.0-sx)*(now.0-sx) + (now.1-sy)*(now.1-sy)) as f64).sqrt() / s as f64;
                time += (((tx-sx)*(tx-sx) + (ty-sy)*(ty-sy)) as f64).sqrt() / t as f64;
                now = (tx, ty);
            }
            ans = ans.min(time);
        }
    }
    println!("{}", ans);
}
