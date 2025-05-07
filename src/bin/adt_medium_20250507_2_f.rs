use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }
    let h: Vec<usize> = (0..h1).into_iter().collect();
    let w: Vec<usize> = (0..w1).into_iter().collect();
    for comb1 in h.iter().combinations(h2) {
        for comb2 in w.iter().combinations(w2) {
            let mut ans = "Yes";
            for (i2, &&i1) in comb1.iter().enumerate() {
                for (j2, &&j1) in comb2.iter().enumerate() {
                    if a[i1][j1] != b[i2][j2] {
                        ans = "No";
                        break;
                    }
                }
                if ans == "No" {
                    break;
                }
            }
            if ans == "Yes" {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}