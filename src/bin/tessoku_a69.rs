use proconio::{input, marker::Chars};

fn maxflow(s: usize, t: usize, v: &Vec<usize>, e: &mut Vec<Vec<usize>>) -> usize {
    loop {
        // 可能な経路を探す
        fn dfs(s: usize, t: usize, e: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> (Vec<usize>, usize) {
            for (i, &f) in e[s].iter().enumerate() {
                if f == 0 || visited[i] { continue; }
                visited[i] = true;
                if i == t {
                    return (vec![t, s], f);
                }

                let (mut path, mut flow) = dfs(i, t, e, visited);
                if !path.is_empty() {
                    path.push(s);
                    flow = flow.min(f);
                    return (path, flow);
                }
            }

            (Vec::new(), 0)
        }

        let mut visited = vec![false; v.len()];
        visited[0] = true;
        let (mut path, flow) = dfs(s, t, &e, &mut visited);
        if flow == 0 { break; }

        // 残差ネットワークを更新する
        path.reverse();
        fn update(path: Vec<usize>, flow: usize, e: &mut Vec<Vec<usize>>) {
            for i in 0..(path.len()-1) {
                let i1 = path[i];
                let i2 = path[i+1];
                e[i1][i2] -= flow;
                e[i2][i1] += flow;
            }
        }

        // println!("path: {:?}, flow: {}, e: {:?}", path, flow, e);
        update(path, flow, e);
    }

    let mut flow = 0;
    for f in e[t].iter() {
        flow += f;
    }

    flow
}

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }

    let v: Vec<usize> = (0..(2*(n+1))).collect();
    let mut e: Vec<Vec<usize>> = vec![vec![0; 2*(n+1)]; 2*(n+1)];
    for j in 0..n {
        e[0][j+1] = 1;
        e[n+j+1][2*n+1] = 1;
    }
    for i in 0..n {
        for j in 0..n {
            if c[i][j] == '#' {
                e[i+1][n+j+1] = 1;
            }
        }
    }

    let ans = maxflow(0, 2*n+1, &v, &mut e);

    println!("{}", ans);
}