use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    // 連結を求める
    let mut dsu = Dsu::new(n);
    let mut edge: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        edge[a-1].push((b-1, i));
        edge[b-1].push((a-1, i));
        dsu.merge(a-1, b-1);
    }

    // 付け替えていいケーブルのリストを返す(自己ループするケーブル)
    fn dfs(a: usize, visited: &mut Vec<bool>, used: &mut Vec<bool>, edge: &Vec<Vec<(usize, usize)>>) -> Option<Vec<usize>> {
        let mut ret: Vec<usize> = Vec::new();
        for &(b, i) in edge[a].iter() {
            if used[i] { continue; }
            used[i] = true;
            if visited[b] {
                ret.push(i);
            } else {
                visited[b] = true;
                if let Some(mut tmp) = dfs(b, visited, used, edge) {
                    if ret.len() < tmp.len() {
                        std::mem::swap(&mut ret, &mut tmp);
                    }
                    ret.extend(tmp);
                };
            }
        }
        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    }

    // DFSで連結の自己ループを見つけて別の連結につなげる
    let mut unconnected: Vec<usize> = Vec::new();
    for group in dsu.groups().iter() {
        unconnected.push(dsu.leader(group[0]));
    }
    let mut visited: Vec<bool> = vec![false; n];
    let mut used: Vec<bool> = vec![false; m];
    let mut ans: Vec<(usize, usize, usize)> = Vec::new();
    let mut cnt = unconnected.len()-1;
    // println!("cnt: {}", cnt);
    for i in 0..n {
        if visited[i] { continue; }
        visited[i] = true;
        if let Some(mut loops) = dfs(i, &mut visited, &mut used, &edge) {
            // println!("i: {}, unconnected: {:?}", i, unconnected);
            let mut tmp: Vec<usize> = Vec::new();
            while cnt > 0 {
                if let Some(k) = loops.pop() {
                    let j = unconnected.pop().unwrap();
                    // println!("k: {}, j: {}, i: {}, leader(i): {}, cnt: {}, root: {}", k, j, i, dsu.leader(i), cnt, root);
                    if dsu.same(i, j) { tmp.push(j);loops.push(k);continue; }
                    // 別の連結成分に付け替える
                    let (a, _) = ab[k];
                    ans.push((k+1, a, j+1));
                    dsu.merge(i, j);
                    cnt -= 1;
                } else {
                    break;
                }
            }
            unconnected.extend(tmp);
            if cnt == 0 { break; }
        };
    }

    // 出力
    println!("{}", ans.len());
    for a in &ans {
        println!("{} {} {}", a.0, a.1, a.2);
    }
}