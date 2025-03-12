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
    for h_perm in (0..h1).into_iter().combinations(h2) {
        let mut a2: Vec<Vec<usize>> = Vec::new();
        for &i in h_perm.iter() {
            a2.push(a[i].clone());
        }
        for w_perm in (0..w1).into_iter().combinations(w2) {
            let mut a3: Vec<Vec<usize>> = vec![vec![0; w2]; h2];
            for (i, ai) in a2.iter().enumerate() {
                for (j, &j2) in w_perm.iter().enumerate() {
                    a3[i][j] = ai[j2];
                }
            }
            if b == a3 {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }
    println!("No");
}