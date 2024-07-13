use proconio::input;
use ac_library::Dsu;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n-1],
    }
    let mut nodes: Vec<usize> = Vec::new();
    let mut dsu = Dsu::new(n+1);
    for (ui, vi) in uv.iter() {
        if ui == &1 || vi == &1 {
            nodes.push(*ui.max(vi));
            continue;
        }
        dsu.merge(*ui, *vi);
    }
    let mut ans = 0;
    let mut max_nodes = 0;
    if nodes.len() == 1 {
        ans = 1;
    } else {
        for ni in nodes.iter() {
            ans += dsu.size(*ni);
            max_nodes = max_nodes.max(dsu.size(*ni));
        }
        ans -= max_nodes;
        ans += 1;
    }
    println!("{}", ans);
}