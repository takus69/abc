use proconio::input;
use ac_library::Dsu;
use std::collections::{VecDeque, HashSet};

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut dsu = Dsu::new(n+1);
    let mut black: Vec<bool> = vec![false; n+1];
    let mut black_cnt: Vec<isize> = vec![0; n+1];
    let mut ans: isize = 0;
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        // println!("u: {}, v: {}, black: {:?}", u, v, black);
        edge[u].push(v);
        edge[v].push(u);
        if dsu.same(u, v) {
            if black[u] == black[v] {
                ans = -1;
            }
        } else {
            let s1 = dsu.size(u) as isize;
            let s2 = dsu.size(v) as isize;
            let cnt1 = black_cnt[dsu.leader(u)];
            let cnt2 = black_cnt[dsu.leader(v)];
            let before = cnt1.min(s1-cnt1) + cnt2.min(s2-cnt2);
            dsu.merge(u, v);
            if black[u] == black[v] {
                // マージテクで小さい方の色を反転する
                let mut que: VecDeque<usize> = VecDeque::new();
                let mut visited: HashSet<usize> = HashSet::new();
                let (s, ss, cnt2, cnt1) = if s1 >= s2 { (v, s2, cnt2, cnt1) } else { (u, s1, cnt1, cnt2) };

                black_cnt[dsu.leader(u)] = cnt1 + ss - cnt2;

                black[s] = !black[s];
                que.push_front(s);
                visited.insert(u);visited.insert(v);
                while let Some(i) = que.pop_back() {
                    for &j in edge[i].iter() {
                        if visited.contains(&j) { continue; }
                        que.push_front(j);
                        visited.insert(j);
                        black[j] = !black[j];
                    }
                }
            } else {
                black_cnt[dsu.leader(u)] = cnt1 + cnt2;
            }
            let cnt = black_cnt[dsu.leader(u)];
            let after = cnt.min(dsu.size(u) as isize - cnt);
            if ans >= 0 {
                ans += after - before;
            }
        }
        println!("{}", ans);
    }
}