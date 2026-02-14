use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hw: [(usize, usize); n],
    }
    let mut h_map: HashMap::<usize, Vec<usize>> = HashMap::new();
    let mut w_map: HashMap::<usize, Vec<usize>> = HashMap::new();
    for (i, &(hi, wi)) in hw.iter().enumerate() {
        let e = h_map.entry(hi).or_insert(Vec::new());
        e.push(i);
        let e = w_map.entry(wi).or_insert(Vec::new());
        e.push(i);
    }
    // println!("h_map: {:?}", h_map);
    // println!("w_map: {:?}", w_map);

    let (mut max_h, mut max_w) = (h, w);
    let (mut now_h, mut now_w) = (0, 0);
    let mut cnt = 0;
    let mut done: Vec<bool> = vec![false; n];
    let mut ans: Vec<(usize, usize)> = vec![(usize::MAX, usize::MAX); n];
    while cnt < n {
        if h_map.contains_key(&max_h) {
            for &i in h_map.get(&max_h).unwrap().iter() {
                if done[i] { continue; }
                done[i] = true;
                let (hi, wi) = hw[i];
                ans[i] = (now_h, now_w);
                now_w += wi;
                cnt += 1;
                // println!("{}: (h, w)=({}, {}), max (h, w)=({}, {}), now (h, w)=({}, {})", i, h, w, max_h, max_w, now_h, now_w);
            }
            max_w = w - now_w;
            max_h = usize::MAX
        } else {
            for &i in w_map.get(&max_w).unwrap().iter() {
                if done[i] { continue; }
                done[i] = true;
                let (hi, wi) = hw[i];
                ans[i] = (now_h, now_w);
                now_h += hi;
                cnt += 1;
                // println!("{}: (h, w)=({}, {}), max (h, w)=({}, {}), now (h, w)=({}, {})", i, h, w, max_h, max_w, now_h, now_w);
            }
            max_h = h - now_h;
            max_w = usize::MAX;
        }
        // println!("cnt: {}", cnt);
    }
    
    for &(hi, wi) in ans.iter() {
        println!("{} {}", hi+1, wi+1);
    }
}