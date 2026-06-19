use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    if n != m { println!("0");return; }

    const MOD: usize = 998244353;
    let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in &uv {
        edge[u-1].push(v-1);
        edge[v-1].push(u-1);
    }

    let mut ans = 1;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        if visited[i] { continue; }
        visited[i] = true;
        let (v_cnt, e_cnt)= bfs(i, &mut visited, &edge);
        // println!("i: {}, v_cnt: {}, e_cnt: {}", i, v_cnt, e_cnt);
        if v_cnt*2 == e_cnt {
            ans *= 2;
            ans %= MOD;
        } else {
            ans = 0;
            break;
        }
    }

    println!("{}", ans);

    fn bfs(u: usize, visited: &mut [bool], edge: &[Vec<usize>]) -> (usize, usize) {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut e_cnt = 0;
        let mut v_cnt = 1;
        que.push_back(u);
        while let Some(u) = que.pop_front() {
            for &v in edge[u].iter() {
                e_cnt += 1;
                if visited[v] { continue; }
                visited[v] = true;
                v_cnt += 1;
                que.push_back(v);
            }
        }

        (v_cnt, e_cnt)
    }
}