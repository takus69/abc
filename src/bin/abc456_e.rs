use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            uv: [(usize, usize); m],
            w: usize,
            s: [Chars; n],
        }
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..n {
            graph[i].push(i);
        }
        for &(u, v) in &uv {
            graph[u-1].push(v-1);
            graph[v-1].push(u-1);
        }
        
        let mut graph2: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); w]; n];
        for d in 0..w {
            let next_d = (d+1)%w;
            for i in 0..n {
                for &j in graph[i].iter() {
                    if s[i][d] == 'o' && s[j][next_d] == 'o' {
                        graph2[i][d].push(j);
                    }
                }
            }
        }

        fn dfs(link: &Vec<Vec<Vec<usize>>>, v: usize, d: usize, seen: &mut Vec<Vec<bool>>, finished: &mut Vec<Vec<bool>>) -> bool {
            seen[v][d] = true;
            let w = seen[0].len();
            let next_d = (d+1)%w;
            for &u in link[v][d].iter() {
                if finished[u][next_d] { continue; }
                if seen[u][next_d] || dfs(link, u, next_d, seen, finished) {
                    return true;
                }
            }
            finished[v][d] = true;

            false
        }

        let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; n];
        let mut finished: Vec<Vec<bool>> = vec![vec![false; w]; n];
        let mut flg = false;
        for i in 0..n {
            if seen[i][0] { continue; }
            if dfs(&graph2, i, 0, &mut seen, &mut finished) {
                flg = true;
                break;
            }
        }
        if flg {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}