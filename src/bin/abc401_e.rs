use proconio::input;
use itertools::Itertools;
use std::collections::{BinaryHeap};
use std::cmp::{Reverse};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(u, v) in uv.iter() {
        edge[u].push(v);
        edge[v].push(u);
    }
    let mut reached: Vec<bool> = vec![false; n+1];
    let mut pushed: Vec<bool> = vec![false; n+1];
    let mut frontier: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    frontier.push(Reverse(1));
    pushed[1] = true;
    let mut prefix_reached= 1;
    let mut ans: Vec<isize> = Vec::new();
    for k in 1..=n {
        // 一番小さい頂点を確認
        while let Some(&Reverse(v)) = frontier.peek() {
            if v <= k {
                frontier.pop();
                if !reached[v] {
                    reached[v] = true;
                    // 次の頂点を入れる
                    for &u in edge[v].iter() {
                        if pushed[u] { continue; }
                        frontier.push(Reverse(u));
                        pushed[u] = true;
                    }
                    while prefix_reached < k {
                        if reached[prefix_reached+1] {
                            prefix_reached += 1;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
        // println!("k: {}, {:?}", k, vertical);
        
        // kまですべて見つかっていたら、今つながっている頂点数を削除すると実現可能
        if prefix_reached== k {
            ans.push(frontier.len() as isize);
        } else {
            ans.push(-1);
        }
    }

    println!("{}", ans.iter().join(" "));
}