use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: usize,
            ab: [(usize, usize); n-1],
        }


        let mut edge: Vec<Vec<usize>> = vec![Vec::new(); n+1];
        for (a, b) in ab {
            edge[a].push(b);
            edge[b].push(a);
        }

        println!("{}", dfs(1, 0, &edge).0);

        fn dfs(to: usize, from: usize, edge: &[Vec<usize>]) -> (usize, usize) {
            let mut nexts: Vec<usize> = Vec::new();
            let mut ans = 1;
            for &next in edge[to].iter() {
                if next == from { continue; }
                let (ans2, up) = dfs(next, to, edge);
                ans = ans.max(ans2);
                nexts.push(up);
            }
            if nexts.is_empty() {
                return (ans, 0);
            }
            nexts.sort();nexts.reverse();

            let mut up = 0;
            if edge[to].len() > 3 {
                ans = ans.max(nexts[0]+nexts[1]+1);
                up = nexts[0]+1;
            } else if edge[to].len() == 3 {
                ans = ans.max(nexts[0]+1);
                up = 1;
            }

            (ans, up)
        }

    }
}