use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut wp: [(usize, usize); n],
        }

        let mut sum_p = 0;
        for (i, &(wi, pi)) in wp.iter().enumerate() {
            sum_p += pi;
        }
        wp.sort_by(|(w1, p1), (w2, p2)| (w1+p1).cmp(&(w2+p2)));
        let mut ans = 0;
        let mut sum_wp = 0;
        for &(wi, pi) in wp.iter() {
            sum_wp += pi+wi;
            if sum_p < sum_wp {
                break;
            }
            ans += 1;
        }

        println!("{}", ans);
    }
}