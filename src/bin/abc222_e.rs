use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        a: [usize; m],
        uv: [(usize, usize); n-1],
    }
    let mut edge: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n+1];
    for (i, &(u, v)) in uv.iter().enumerate() {
        edge[u].push((v, i));
        edge[v].push((u, i));
    }
    let mut cnt: Vec<usize> = vec![0; n-1];
    let mut current = a[0];
    for &next in &a {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; n+1];
        let mut parent: Vec<(usize, usize)> = vec![(0, 0); n+1];
        visited[current] = true;
        que.push_back(current);
        while let Some(b) = que.pop_front() {
            for &(c, i) in &edge[b] {
                if visited[c] { continue; }
                visited[c] = true;
                que.push_back(c);
                parent[c] = (b, i);
            }
        }
        // 経路復元
        let mut b = next;
        while b != current {
            let (c, i) = parent[b];
            cnt[i] += 1;
            b = c;
        }
        current = next;
    }
    // println!("cnt: {:?}", cnt);
    
    const MOD: usize = 998244353;
    let k = k.abs() as usize;
    let sum: usize = cnt.iter().sum();
    // println!("sum: {}, k: {}", sum, k);
    if (sum+k)%2 == 1 { println!("0");return; }
    let k = (sum+k)/2;
    let mut dp: Vec<usize> = vec![0; k+1];
    dp[0] = 1;
    for &c1 in &cnt {
        let mut next_dp: Vec<usize> = dp.clone();
        for (i, &c2) in dp.iter().enumerate() {
            if i+c1 > k { break; }
            next_dp[i+c1] += c2;
            next_dp[i+c1] %= MOD;
        }
        dp = next_dp;
    }
    // println!("dp: {:?}", dp);
    println!("{}", dp[k]);
}