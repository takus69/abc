use proconio::input;
use ac_library::SccGraph;

/// 以下の2つの解法
/// 1. サイクルから逆にDFSして深さを計算
/// 2. 強連結成分分解
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut g = SccGraph::new(n);
    for (i, ai) in a.iter().enumerate() {
        g.add_edge(i, ai-1);

    }
    let mut scc = g.scc();
    scc.reverse();
    let mut r = vec![0; n];
    for s in scc.iter() {
        // サイクルか、外に出るか(出字数1のため、サイクルは最後にのみ来る)
        if s.len() > 1 || s[0] == a[s[0]]-1 {
            for u in s.iter() {
                r[*u] = s.len();
            }
        } else {
            r[s[0]] = r[a[s[0]]-1] + 1;
        }
    }
    let ans: usize = r.iter().sum();
    println!("{}", ans);
}