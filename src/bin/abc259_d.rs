use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        s: (i64, i64),
        t: (i64, i64),
        xyr: [(i64, i64, i64); n],
    }
    let mut e = HashMap::new();
    for i in 0..n {
        e.insert(i, Vec::new());
    }
    let mut si = 0;
    let mut ti = 0;
    for i in 0..n {
        let (xi, yi, ri) = xyr[i];
        let (sx, sy) = s;
        let (tx, ty) = t;
        if  (xi-sx)*(xi-sx) + (yi-sy)*(yi-sy) == ri*ri {
            si = i;
        }
        if  (xi-tx)*(xi-tx) + (yi-ty)*(yi-ty) == ri*ri {
            ti = i;
        }

        for j in (i+1)..n {
            let (xj, yj, rj) = xyr[j];
            let dis = (xi-xj)*(xi-xj) + (yi-yj)*(yi-yj);
            if dis <= (ri+rj)*(ri+rj) && dis >= (ri-rj)*(ri-rj) {
                e.get_mut(&i).unwrap().push(j);
                e.get_mut(&j).unwrap().push(i);
            }
        }
    }

    // BFS
    let mut queue = VecDeque::new();
    queue.push_back(si);
    let mut ans = "No";
    if si == ti { ans = "Yes"; }
    let mut visited = vec![false; n];
    visited[si] = true;
    while !queue.is_empty() {
        let i = queue.pop_front().unwrap();
        for j in e[&i].iter() {
            if visited[*j] { continue; }
            if j == &ti {
                ans = "Yes";
                break;
            }
            visited[*j] = true;
            queue.push_back(*j);
        }
    }

    // println!("si: {}, ti: {}, e: {:?}", si, ti, e);
    println!("{}", ans);
}