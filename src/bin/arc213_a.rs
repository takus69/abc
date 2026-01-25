use proconio::input;
use ac_library::FenwickTree;
use std::collections::{HashMap, BinaryHeap};

fn main() {
    input! {
        n: usize,
        l: usize,
        cp: [[usize; l+1]; n],
    }
    let mut p: Vec<Vec<usize>> = Vec::new();
    p.push((1..=l).collect());
    let mut c: Vec<usize> = Vec::new();
    c.push(0);
    for cpi in cp.iter() {
        c.push(cpi[0]);
        p.push(cpi[1..=l].to_vec());
    }

    // 転倒数
    fn inversion(p: &Vec<usize>, q: &Vec<usize>) -> usize {
        let l = p.len();
        let mut fw: FenwickTree<usize> = FenwickTree::new(l, 0);
        let mut cnt = 0;

        let mut pp: Vec<usize> = vec![0; l+1];
        for (i, &pi) in p.iter().enumerate() {
            pp[pi] = i;
        }

        for (i, &qi) in q.iter().enumerate() {
            let j = pp[qi];
            cnt += j-fw.sum(0..j);
            fw.add(j, 1);
        }

        cnt
    }
    
    // let mut dp: HashMap<usize, usize> = HashMap::new();  // key: i, value: money
    // dp.insert(0, 0);
    let mut dp: BinaryHeap<(usize, usize)> = BinaryHeap::new();  // (money, i)
    dp.push((0, 0));
    for i in 1..=n {
        let pi = &p[i];
        // for (&j, &m) in dp.clone().iter() {
        let mut tmp: BinaryHeap<(usize, usize)> = BinaryHeap::new();
        while let Some((m, j)) = dp.pop() {
            let inv = inversion(pi, &p[j]);
            let opt_v = 0;
            tmp.push((m, j));
            if inv <= j.abs_diff(i) {
                let v = m+c[i];
                // let e = dp.entry(i).or_insert(v);
                // if v > *e {
                //     *e = v;
                // }
                tmp.push((v, i));
                break;
            }
        }
        while let Some((m, j)) = tmp.pop() {
            dp.push((m, j));
        }
    }

    // let mut ans = 0;
    // for (_, &m) in dp.iter() {
    //     ans = ans.max(m);
    // }
    let (ans, _) = dp.pop().unwrap();
    println!("{}", ans);
}