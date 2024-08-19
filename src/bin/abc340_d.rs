use proconio::input;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

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

        while !heap.is_empty() {
            let (Reverse(dis), a) = heap.pop().unwrap();
            // println!("a: {}, dis: {}", a, dis);
            for (b, d) in edge.get(&a).unwrap().iter() {
                let cost = dis + d;
                if cost < distance[b] {
                    distance.insert(*b, cost);
                    parent[*b] = a;
                    heap.push((Reverse(cost), *b));
                }
            }
        }

        Self { distance, parent }
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut edge: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..(n-1) {
        input! {
            ai: usize,
            bi: usize,
            xi: usize,
        }
        edge.insert(i, vec![(i+1, ai), (xi-1, bi)]);
    }
    edge.insert(n-1, vec![]);
    let dijkstra = Dijkstra::new(n, edge, 0);
    println!("{}", dijkstra.distance[&(n-1)]);
}