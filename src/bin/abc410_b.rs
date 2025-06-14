use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut ans: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = vec![0; n];
    for &xi in x.iter() {
        if xi > 0 {
            ans.push(xi);
            b[xi-1] += 1;
        } else {
            let mut min_cnt = usize::MAX;
            let mut min_i = 0;
            for (i, &bi) in b.iter().enumerate() {
                if min_cnt > bi {
                    min_cnt = bi;
                    min_i = i;
                }
            }
            ans.push(min_i+1);
            b[min_i] += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}