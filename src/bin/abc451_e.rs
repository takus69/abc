use proconio::input;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
    }
    let mut a: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; n];
    for i in 0..(n-1) {
        input! {
            ai: [usize; n-i-1],
        }
        for j in 0..(n-i-1) {
            a[i][j+i+1] = ai[j];
            a[j+i+1][i] = ai[j];
        }
    }
    let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    let mut connected: HashSet<usize> = HashSet::new();
    connected.insert(0);
    for (i, &ai) in a[0].iter().enumerate() {
        heap.push((Reverse(ai), i, 0));
    }
    let mut edge: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..(n-1) {
        // println!("heap: {:?}", heap);
        while let Some((Reverse(c), v, u)) = heap.pop() {
            if connected.contains(&v) { continue; }
            connected.insert(v);
            edge[v].push((u, c));
            edge[u].push((v, c));
            for (i, &ai) in a[v].iter().enumerate() {
                heap.push((Reverse(ai), i, v));
            }
            break;
        }
        // println!("edge: {:?}", edge);
    }
    // println!("edge: {:?}", edge);

    // check
    for s in 0..n {
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; n];
        que.push_front((s, 0));
        visited[s] = true;
        while let Some((v, c)) = que.pop_back() {
            for &(i, ai) in edge[v].iter() {
                if visited[i] { continue; }
                // println!("{}=>{}, {}, a: {}", s, i, c+ai, a[s][i]);
                if c+ai != a[s][i] {
                    println!("No");
                    return;
                }
                que.push_front((i, c+ai));
                visited[i] = true;
            }
        }
    }

    println!("Yes");
}