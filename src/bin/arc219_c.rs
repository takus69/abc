use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut h_rooms: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut hs: HashSet<usize> = HashSet::new();
    let mut cost1 = 0;  // 右エレベーターを使わない
    let mut cost2 = 0;  // 右エレベーターを使う
    let mut min_cost_cnt = 0;
    let mut max_cost = (0, 0);
    let mut above_cost = usize::MAX;
    for &(a, b) in &ab {
        let e = h_rooms.entry(a).or_insert(vec![1, w]);
        e.push(b);
        hs.insert(a);
    }
    for a in &hs {
        let e = h_rooms.get_mut(a).unwrap();
        e.sort();
        
        // 右のエレベーターを使わない
        cost1 += (e[e.len()-2]-1)*2;

        // 右のエレベーターを使う
        let mut pre = 1;
        let mut min_cost = w-1;
        let mut update_flg = false;
        let mut tmp_above_cost = usize::MAX;
        for i in 0..e.len() {
            let cost = (pre-1)*2 + (w-e[i])*2;
            if cost <= min_cost {
                min_cost = min_cost.min(cost);
                update_flg = true;
            } else {
                tmp_above_cost = tmp_above_cost.min(cost)
            }
            pre = e[i];
        }
        if !update_flg {
            min_cost_cnt += 1;
            above_cost = above_cost.min(tmp_above_cost);
        } else {
            let (max_cost1, max_cost2) = max_cost;
            if max_cost1 < min_cost {
                max_cost = (min_cost, max_cost1);
            } else if max_cost2 < min_cost {
                max_cost = (max_cost1, min_cost);
            }
        }
        cost2 += min_cost;
        // println!("a: {}, cost1: {}, cost2: {}, max_cost: {:?}", a, cost1, cost2, max_cost);
    }
    // println!("min_cost_cnt: {}, min_cost_eq_cnt: {}, max_cost: {:?}", min_cost_cnt, min_cost_eq_cnt, max_cost);
    let mut cost3 = usize::MAX;
    if min_cost_cnt%2==1 {
        if min_cost_cnt > 2 && above_cost != usize::MAX {
            cost3 = cost2 + above_cost - (w-1);
        }
        cost2 += w-1;
        cost2 -= max_cost.0;
    } else if min_cost_cnt==0 {
        cost2 += 2*(w-1);
        cost2 -= max_cost.0 + max_cost.1;
    }
    println!("{}", cost1.min(cost2).min(cost3));
}