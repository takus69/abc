use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    }
    ab.sort();
    for perm in (0..n).permutations(n) {
        let mut conv_cd: Vec<(usize, usize)> = Vec::new();
        for &(c, d) in &cd {
            let mut c2 = perm[c-1]+1;
            let mut d2 = perm[d-1]+1;
            if c2 > d2 { std::mem::swap(&mut c2, &mut d2); }
            conv_cd.push((c2, d2));
        }
        conv_cd.sort();
        if ab == conv_cd {
            println!("Yes");
            return;
        }
    }
    println!("No");
}