use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }
    let mut cnt: HashMap<Vec<Vec<usize>>, usize> = HashMap::new();
    cnt.insert(a.clone(), 0);
    let mut que: VecDeque<Vec<Vec<usize>>> = VecDeque::new();
    que.push_front(a.clone());

    while !que.is_empty() {
        let t = que.pop_back().unwrap();
        let c = cnt[&t];
        for i in 0..(h-1) {
            let mut t2 = t.clone();
            let tmp = t2[i].clone();
            t2[i] = t2[i+1].clone();
            t2[i+1] = tmp;
            if !cnt.contains_key(&t2) {
                que.push_front(t2.clone());
                cnt.insert(t2, c+1);
            }
        }
        for j in 0..(w-1) {
            let mut t2 = t.clone();
            for i in 0..h {
                let tmp = t2[i][j];
                t2[i][j] = t2[i][j+1];
                t2[i][j+1] = tmp;
            }
            if !cnt.contains_key(&t2) {
                que.push_front(t2.clone());
                cnt.insert(t2, c+1);
            }
        }
    }
    if cnt.contains_key(&b) {
        println!("{}", cnt[&b]);
    } else {
        println!("-1");
    }
}