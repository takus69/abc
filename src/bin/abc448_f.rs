use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let w = 245;
    let ww = 20_000_000 / w + 1;
    let mut r#box: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); w];
    for (ii, &(x, y)) in xy.iter().enumerate() {
        let mut i = x/ww;
        if i == w { i -= 1; }
        r#box[i].push((x, y, ii+1));
    }
    for i in 0..w {
        r#box[i].sort_by(|a, b| a.1.cmp(&b.1));
        if i%2 == 1 {
            r#box[i].reverse();
        }
    }
    let mut r#box2: Vec<(usize, usize, usize)> = Vec::new();
    for b in r#box {
        r#box2.extend(b);
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        let (_, _, ii) = r#box2[i];
        ans.push(ii);
    }
    for (i, &p) in ans.iter().enumerate() {
        if p == 1 {
            println!("{}", ans[i..n].iter().chain(ans[0..i].iter()).join(" "));
            return;
        }
    }
}