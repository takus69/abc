use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = 1 << 61;

struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize, a: Vec<usize>) -> Self {
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                // 始点は0
                // BinaryHeapはpopで最大値を取得するので、Reverse()で最小値を取得できるようにする
                heap.push((Reverse(a[0]), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            // println!("target: {}, cost: {}", target, distance[target]);
            if distance[target] < d {
                // 既にもっと短い経路が見つかってるので無視
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost + a[next] {
                    // 短い経路の候補となるので処理を行う
                    distance[next] = d + cost + a[next];
                    heap.push((Reverse(distance[next]), next));
                    // ひとつ前の頂点を保存しておく
                    parent[next] = target;
                }
                // println!("next: {}, cost: {}", next, distance[next]);
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uvb: [(usize, usize, usize); m],
    }
    let mut edge: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (u, v, b) in uvb.iter() {
        edge[u-1].push((v-1, *b));
        edge[v-1].push((u-1, *b));
    }
    let a0 = a[0];
    let mut dijk = Dijkstra::new(n, edge, 0, a);
    let mut ans: Vec<usize> = Vec::new();
    for i in 1..n {
        ans.push(dijk.distance(i));
    }

    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
