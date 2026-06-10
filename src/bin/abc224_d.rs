use proconio::input;
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        m: usize,
        uv: [(usize, usize); m],
        p: [usize; 8],
    }
    
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); 9];
    for &(u, v) in &uv {
        edge[u-1].push(v-1);
        edge[v-1].push(u-1);
    }
    let mut set: HashSet<Vec<usize>> = HashSet::new();
    
    let mut state: Vec<usize> = vec![usize::MAX; 9];
    for (i, &pi) in p.iter().enumerate() {
        state[pi-1] = i;
    }
    let goal: Vec<usize>= vec![0, 1, 2, 3, 4, 5, 6, 7, usize::MAX];
    let mut que: VecDeque<(Vec<usize>, usize)> = VecDeque::new();
    que.push_back((state.clone(), 0));
    set.insert(state.clone());

    while let Some((state, cnt)) = que.pop_front() {
        if state == goal {
            println!("{}", cnt);
            std::process::exit(0);
        }
        let mut u = usize::MAX;
        for (i, &v) in state.iter().enumerate() {
            if v == usize::MAX {
                u = i;
            }
        }
        for &v in &edge[u] {
            let mut next_state = state.clone();
            let value = next_state[v];
            next_state[u] = value;
            next_state[v] = usize::MAX;
            if set.contains(&next_state) {
                continue;
            }
            que.push_back((next_state.clone(), cnt+1));
            set.insert(next_state.clone());
        }

    }
    println!("-1");
}
