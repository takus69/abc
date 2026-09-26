use proconio::input;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

/// ダイクストラ法
/// # 引数
/// *`n` - 頂点数
/// * `edge` - 重みをもった連結リストVec<頂点, 重み>
/// * `init` - 開始位置
pub struct Dijkstra {
    pub distance: HashMap<usize, usize>,
    pub parent: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edge: HashMap<usize, Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            distance.insert(i, usize::MAX);
        }
        distance.insert(init, 0);
        let mut parent: Vec<usize> = vec![usize::MAX; n];
        let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
        heap.push((Reverse(0), init));

        while let Some((Reverse(dis), a)) = heap.pop() {
            if let Some(neighbors) = edge.get(&a) {
                for (b, d) in neighbors.iter() {
                    let cost = dis + d;
                    if cost < distance[b] {
                        distance.insert(*b, cost);
                        parent[*b] = a;
                        heap.push((Reverse(cost), *b));
                    }
                }
            }
        }

        Self { distance, parent }
    }
}


fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut edge: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..n {
        let e = edge.entry(i).or_insert(Vec::new());
        let j = (i+1)%n;
        e.push((j, a[i]));
        e.push((n, b[i]));
        let e = edge.entry(j).or_insert(Vec::new());
        e.push((i, a[i]));
        let e = edge.entry(n).or_insert(Vec::new());
        e.push((i, b[i]));
    }
    let dijkstra = Dijkstra::new(n+1, edge, n);

    let mut dist1: Vec<usize> = vec![0; 2*n+1];
    for i in 0..(2*n) {
        dist1[i+1] = dist1[i] + a[i%n];
    }

    for _ in 0..q {
        input! {
            mut s: usize,
            mut t: usize
        }
        s -= 1;
        t -= 1;

        if s == n || t == n {
            let ans = dijkstra.distance.get(&(s.min(t))).unwrap();
            println!("{}", ans);
            continue;
        }

        let mut ans = dijkstra.distance.get(&s).unwrap() + dijkstra.distance.get(&t).unwrap();
        let tmp = if s > t {
            (dist1[s] - dist1[t]).min(dist1[t+n] - dist1[s])
        } else {
            (dist1[t] - dist1[s]).min(dist1[s+n] - dist1[t])
        };
        println!("{}", ans.min(tmp));
    }
}