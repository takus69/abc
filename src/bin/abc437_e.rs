use proconio::input;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut nodes: Vec<HashMap<usize, (usize, Vec<usize>)>> = vec![HashMap::new()];
    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(0, 0);
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        let j = nodes.len();
        let &xi2 = map.get(&xi).unwrap();
        if !nodes[xi2].contains_key(&yi) {
            nodes.push(HashMap::new());
        }
        let e = nodes[xi2].entry(yi).or_insert((j, Vec::new()));
        map.insert(i+1, e.0);
        e.1.push(i+1);
    }

    for m in nodes.iter_mut() {
        for (_, (_, e)) in m.iter_mut() {
            e.sort();
        }
    }
    
    fn dfs(i: usize, nodes: &Vec<HashMap<usize, (usize, Vec<usize>)>>, ans: &mut Vec<usize>) {
        let child = &nodes[i];
        if !child.is_empty() {
            let mut keys: Vec<usize> = child.clone().into_keys().collect();
            keys.sort();
            for k in keys.iter() {
                let (j, v) = child.get(k).unwrap();
                for &i in v.iter() {
                    ans.push(i);
                }
                dfs(*j, nodes, ans);
            }
        }
    }

    let mut ans: Vec<usize> = Vec::new();
    dfs(0, &nodes, &mut ans);
    
    println!("{}", ans.iter().join(" "));
}