use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mg: usize,
        mut uv: [(usize, usize); mg],
        mh: usize,
        mut ab: [(usize, usize); mh],
        a: [usize; n*(n-1)/2],
    }
    for i in 0..mg {
        let (u, v) = uv[i];
        uv[i] = (u-1, v-1);
    }
    for i in 0..mh {
        let (a, b) = ab[i];
        ab[i] = (a-1, b-1);
    }
    let mut map_a: Vec<Vec<usize>> = vec![vec![]; n-1];
    let mut cnt = 0;
    for i in 0..(n-1) {
        for _ in 0..(n-i-1) {
            map_a[i].push(a[cnt]);
            cnt += 1;
        }
    }
    let mut map_g: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        map_g.insert(i, vec![]);
    }
    for (u, v) in uv.iter() {
        let e = map_g.entry(*u).or_insert(vec![]);
        e.push(*v);
    }
    let mut map_h: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        map_h.insert(i, vec![]);
    }
    for (u, v) in ab.iter() {
        let e = map_h.entry(*u).or_insert(vec![]);
        e.push(*v);
    }

    let mut opt_ans = usize::MAX;
    for perm in (0..n).permutations(n) {
        let mut perm2 = vec![0; n];
        for i in 0..n {
            let mut a = perm[i];
            perm2[a] = i;
        }
        let mut ans = 0;
        // 追加
        for (u, v) in uv.iter() {
            let mut a = perm[*u];
            let mut b = perm[*v];
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            if !map_h.get(&a).unwrap().contains(&b) {
                ans += map_a[a][b-a-1];
            }
        }
        // 削除
        for (a, b) in ab.iter() {
            let mut u = perm2[*a];
            let mut v = perm2[*b];
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            if !map_g.get(&u).unwrap().contains(&v) {
                ans += map_a[*a][b-a-1];
            }
        }
        if opt_ans > ans {
            opt_ans = ans;
        }
    }
    println!("{}", opt_ans);
}