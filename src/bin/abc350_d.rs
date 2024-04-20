use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut map = HashMap::new();
    for i in 1..=n {
        map.insert(i, vec![]);
    }
    let mut visited = vec![false; n];
    for (a, b) in &ab {
        let m = map.get_mut(a).unwrap();
        m.push(*b);
        let m = map.get_mut(b).unwrap();
        m.push(*a);
    }
    let mut ans: i64 = 0;
    for a in 1..=n {
        let mut cnt = 0;
        let mut cnt2 = 0;
        if !visited[a-1] {
            let mut queue = VecDeque::new();
            queue.push_back(a);
            visited[a-1] = true;
            cnt += 1;
            while !queue.is_empty() {
                let q = queue.pop_front().unwrap();
                for b in map.get(&q).unwrap() {
                    cnt2 += 1;
                    if !visited[*b-1] {
                        queue.push_back(*b);
                        visited[b-1] = true;
                        cnt += 1;
                    }
                }
            }
        }
        ans += (cnt * (cnt - 1) / 2) - (cnt2 / 2);
        // println!("a: {}, cnt: {}, cnt2: {}", a, cnt, cnt2);
    }
    // println!("{:?}", map);
    println!("{}", ans);
}