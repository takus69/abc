use proconio::input;
use std::collections::{VecDeque, HashMap};

fn main() {
    input! {
        n: usize,
        x: [isize; n],
        uvw: [(usize, usize, usize); n-1],
    }
    let mut link: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for &(u, v, w) in uvw.iter() {
        let e = link.entry(u).or_insert(Vec::new());
        e.push((v, w));

        let e = link.entry(v).or_insert(Vec::new());
        e.push((u, w));
    }
    
    // 葉を探す.根は1
    let mut visited: Vec<bool> = vec![false; n+1];
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_front(1);
    visited[1] = true;
    let mut leaf_que: VecDeque<usize> = VecDeque::new();
    let mut leaf_visited: Vec<bool> = vec![false; n+1];
    while let Some(v) = que.pop_back() {
        let mut cnt = 0;
        for &(u, _) in link[&v].iter() {
            if visited[u] { continue; }
            que.push_front(u);
            cnt += 1;
            visited[u] = true;
        }
        if cnt == 0 {
            leaf_que.push_front(v);
            leaf_visited[v] = true;
        }
    }
    // println!("leaf_que: {:?}", leaf_que);
    // println!("leaf_visited: {:?}", leaf_visited);

    // 葉から順に登っていく
    let mut ans = 0;
    let mut y = vec![0];
    let mut cnt: Vec<usize> = vec![0; n+1];
    y.extend(x);
    while let Some(v) = leaf_que.pop_back() {
        cnt[v] += 1;
        if cnt[v] < link[&v].len()-1 { continue; }
        leaf_visited[v] = true;
        // println!("y: {:?}", y);
        for &(u, w) in link[&v].iter() {
            if leaf_visited[u] { continue; }

            ans += w* y[v].unsigned_abs();
            y[u] += y[v];
            y[v] = 0;
            leaf_que.push_front(u);
            // println!("leaf_visited: {:?}", leaf_visited);
        }
    }
    
    println!("{}", ans);
}