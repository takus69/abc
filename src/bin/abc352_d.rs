use proconio::input;
use ac_library::{Segtree, Max, Min};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut p2: Vec<(usize, usize)> = p.into_iter().enumerate().collect();
    p2.sort_by(|a, b| a.1.cmp(&b.1));
    let mut max_sg = Segtree::<Max<usize>>::new(n + 1);
    let mut min_sg = Segtree::<Min<usize>>::new(n + 1);
    for (ai, i) in p2 {
        max_sg.set(i, ai);
        min_sg.set(i, ai);
    }
    let mut ans = n-1;
    for i in 1..=(n-k+1) {
        let max_i = max_sg.prod(i..i+k);
        let min_i = min_sg.prod(i..i+k);
        ans = ans.min(max_i - min_i);
    }
    println!("{}", ans);
}