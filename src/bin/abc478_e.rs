use proconio::input;
use itertools::Itertools;
use ac_library::SccGraph;

fn main() {
    input! {
        n: usize,
        q: usize,
        tuv: [(usize, usize, usize); q],
    }
    let mut scc = SccGraph::new(n);
    for &(_, u, v) in &tuv {
        scc.add_edge(u-1, v-1);
    }
    let mut ans: Vec<usize> = vec![0; n];
    let mut cnt = 1;
    for set in scc.scc().iter() {
        for &i in set {
            ans[i] = cnt;
        }
        cnt += 1;
    }
    let mut flg = true;
    for &(t, u, v) in &tuv {
        if ans[u-1]+t > ans[v-1] {
            flg = false;
            break;
        }
    }
    if flg {
        println!("Yes");
        println!("{}", ans.iter().join(" "));
    } else {
        println!("No");
    }
}