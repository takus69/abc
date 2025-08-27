use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans: Vec<(usize, usize, usize)> = Vec::new();
    for (i, &ai) in a.iter().enumerate() {
        ans.push((ai, i, 0));
    }
    ans.sort();
    let mut now = 0;
    let mut pre = 0;
    for i in 0..n {
        if ans[i].0 > pre {
            now += 1;
        }
        ans[i].2 = now;
        pre = ans[i].0;
    }
    ans.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{}", ans.iter().map(|a| a.2).collect::<Vec<usize>>().into_iter().join(" "));
}