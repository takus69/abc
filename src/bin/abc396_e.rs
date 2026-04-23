use proconio::input;
use itertools::Itertools;
use std::collections::{VecDeque, HashSet, HashMap};
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(usize, usize, usize); m],
    }
    let mut x: Vec<usize> = Vec::new();
    let mut y: Vec<usize> = Vec::new();
    let mut z: Vec<usize> = Vec::new();
    let mut len = 0;
    for &(xi, yi, zi) in xyz.iter() {
        x.push(xi);
        y.push(yi);
        z.push(zi);
        len = len.max(format!("{:b}", zi).len());
    }
    fn exit() {
        println!("-1");
        std::process::exit(0);
    }
    let mut link: Vec<Vec<(usize, usize)>> = vec![vec![]; n+1];
    let mut dsu = Dsu::new(n+1);
    for i in 0..m {
        let xi = x[i];
        let yi = y[i];
        let zi = z[i];
        link[xi].push((yi, zi));
        link[yi].push((xi, zi));
        dsu.merge(xi, yi);
    }
    let mut leaders: HashSet<usize> = HashSet::new();
    for i in 1..=n {
        leaders.insert(dsu.leader(i));
    }
    let leaders: Vec<usize> = leaders.into_iter().collect();

    let mut ans: Vec<usize> = vec![0; n];
    for i in 0..len {  // 桁ごとに処理
        let mut xors: Vec<usize> = vec![usize::MAX; n+1];
        let mut visited: Vec<bool> = vec![false; n+1];
        for &leader in leaders.iter() {  // 連結成分ごとに処理
            let mut que: VecDeque<usize> = VecDeque::new();
            visited[leader] = true;
            xors[leader] = 0;
            que.push_front(leader);
            while !que.is_empty() {
                let u = que.pop_back().unwrap();
                let xor = xors[u];
                for &(v, zi) in link[u].iter() {
                    let next_xor = xor ^ (zi >> i & 1);
                    if xors[v] == usize::MAX {
                        xors[v] = next_xor;
                    } else if xors[v] != next_xor {
                        exit();
                    }
                    if visited[v] { continue; }
                    visited[v] = true;
                    que.push_front(v);
                }
            }
        }
        // 連結成分ごとに最小の値を求める
        let mut cnt: Vec<usize> = vec![0; n+1];
        for j in 1..=n {
            let leader = dsu.leader(j);
            if xors[j] == 1 {
                cnt[leader] += 1;
            }
        }
        for (j, &xor) in xors.iter().enumerate() {
            if j == 0 { continue; }
            let leader = dsu.leader(j);
            if (xor == 1 && cnt[leader]*2 <= dsu.size(leader)) || (xor == 0 && cnt[leader]*2 > dsu.size(leader)) {
                ans[j-1] += 1 << i;
            }
        }
    }
    
    fn swap_bit(x: usize, len: usize) -> usize {
        let mut res = 0;
        for i in 0..len {
            if x >> i & 1 == 0 {
                res += 1 << i;
            }
        }
        res
    }
    // 連結成分ごとに最小の値を求める
    let mut sum_ans: HashMap<usize, (usize, usize)> = HashMap::new();
    for i in 1..=n {
        let leader = dsu.leader(i);
        let (e0, e1) = sum_ans.entry(leader).or_insert((0, 0));
        // println!("e: ({}, {}), ans: ({}, {})", e0, e1, ans[i-1], swap_bit(ans[i-1], len));
        *e0 += ans[i-1];
        *e1 += swap_bit(ans[i-1], len);
    }
    for i in 1..=n {
        let leader = dsu.leader(i);
        let (e0, e1) = sum_ans.get(&leader).unwrap();
        if e0 > e1 {
            ans[i-1] = swap_bit(ans[i-1], len);
        }
    }

    println!("{}", ans.iter().join(" "));
}