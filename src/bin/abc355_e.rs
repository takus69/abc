use proconio::input_interactive;
use std::collections::{HashMap, VecDeque};

fn main() {
    input_interactive! {
        n: u32,
        mut l: i64,
        mut r: i64,
    }

    let max_r = 1<<n;
    let mut v: Vec<i64> = Vec::new();
    let mut e : HashMap<i64, Vec<i64>> = HashMap::new();

    // 移動可能な頂点と辺を追加
    for i in 0..=n {
        for j in 0..1<<(n-i) {
            let v1 = (1<<i)*j;
            let v2 = (1<<i)*(j+1);
            e.entry(v1).or_default().push(v2);
            e.entry(v2).or_default().push(v1);
            v.push(v1);
            v.push(v2);
        }
        v.sort();
        v.dedup();
    }
    // println!("{:?}, {:?}", v, e);

    // 最短経路をBFSで求める
    let mut q = VecDeque::new();
    let mut visited: HashMap<i64, bool> = HashMap::new();
    let mut path: HashMap<i64, usize> = HashMap::new();
    let mut pre: HashMap<i64, i64> = HashMap::new();
    for v1 in v.iter() {
        visited.insert(*v1, false);
        path.insert(*v1, max_r);
        pre.insert(*v1, -1);
    }
    q.push_back(l);
    visited.insert(l, true);
    path.insert(l, 0);
    while let Some(v1) = q.pop_front() {
        let p = path[&v1];
        for v2 in e[&v1].iter() {
            if !visited[&v2] {
                path.insert(*v2, p+1);
                pre.insert(*v2, v1);
                q.push_back(*v2);
            }
            visited.insert(*v2, true);
        }
    }
    // println!("{:?}, {:?}", path, pre);
    
    // 経路復元
    r += 1;
    let mut r2 = r;
    let mut ans = 0;
    while pre[&r2] > -1 {
        let mut l2 = pre[&r2];
        let ll = l2;
        // println!("l: {}, r: {}", l2, r);
        let mut flg: i64 = 1;
        if l2 > r2 {
            std::mem::swap(&mut l2, &mut r2);
            flg = -1;
        }
        for i in 0..=n {
            // println!("i: {}, l2: {}, r2: {}, {} {}", i, l2, r2, l2/(1<<i)+1, r2/(1<<i));
            if r2 - l2 == 1<<i {
                let j = l2/(1<<i);
                println!("? {} {}", i, j);
                break;
            }
        }
        input_interactive! {
            t: i64,
        };
        ans += (ans + 100 + flg * t)%100;
        r2 = ll;
    }

    println!("! {}", ans);
}
