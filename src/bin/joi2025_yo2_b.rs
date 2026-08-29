use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
        p: [isize; n],
    }
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut ans = -1;
    for (i, &pi) in p.iter().enumerate() {
        if pi == -1 {
            if a[i] <= x {
                que.push_front((i+1, a[i]));
                ans = ans.max((i+1) as isize);
            }
        } else {
            edge[pi as usize].push(i+1);
        }
    }
    while let Some((u, cost)) = que.pop_back() {
        for &v in &edge[u] {
            let cost2 = a[v-1] + cost;
            if cost2 > x { continue; }
            que.push_front((v, cost2));
            ans = ans.max(v as isize);
        }
    }
    println!("{}", ans);
}