use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut pc: Vec<usize> = (0..=n).collect();
    let mut server: (usize, usize) = (0, 0);
    let mut ss: Vec<String> = Vec::new();
    let mut query: Vec<Vec<(bool, usize, usize)>> = vec![Vec::new(); n];
    for _ in 0..q {
        input! {
            c: usize,
            mut p: usize,
        }
        p -= 1;
        match c {
            1 => {
                pc[p] = query.len();
                query.push(vec![(false, server.0, server.1)]);
            },
            2 => {
                input! {
                    s: String,
                }
                query[pc[p]].push((true, ss.len(), 1));
                ss.push(s);
            },
            3 => {
                server = (pc[p], query[pc[p]].len());
            },
            _ => {},
        }
    }
    // println!("server: {:?}, query: {:?}, ss: {:?}, pc: {:?}", server, query, ss, pc);

    fn dfs(i: usize, l: usize, query: &Vec<Vec<(bool, usize, usize)>>, ss: &Vec<String>, ans: &mut Vec<String>) {
        // println!("i: {}, l: {}", i, l);
        for j in 0..l {
            let (b, k, ll) = query[i][j];
            // println!("b: {}, k: {}, ll: {}", b, k, ll);
            if b {
                ans.push(ss[k].clone());
            } else {
                dfs(k, ll, query, ss, ans);
            }
        }
    }

    let mut ans = Vec::new();
    dfs(server.0, server.1, &query, &ss, &mut ans);
    println!("{}", ans.iter().join(""));
}