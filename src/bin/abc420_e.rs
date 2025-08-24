use proconio::input;
use ac_library::Dsu;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut dsu = Dsu::new(n+1);
    let mut black: HashMap<usize, usize> = HashMap::new();
    let mut black2: Vec<bool> = vec![false; n+1];
    for i in 1..=n {
        black.insert(i, 0);
    }
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                u: usize,
                v: usize,
            }
            let u_leader = dsu.leader(u);
            let v_leader = dsu.leader(v);
            if dsu.same(u, v) { continue; }
            dsu.merge(u, v);
            let leader = dsu.leader(u);
            let black_cnt = black.get(&u_leader).unwrap() + black.get(&v_leader).unwrap();
            black.insert(leader, black_cnt);
        } else if c == 2 {
            input! {
                v: usize,
            }
            let leader = dsu.leader(v);
            let mut black_cnt = *black.get(&leader).unwrap();
            if black2[v] {
                black2[v] = false;
                black_cnt -= 1;
            } else {
                black2[v] = true;
                black_cnt += 1;
            }
            black.insert(leader, black_cnt);
        } else {
            input! {
                v: usize,
            }
            let leader = dsu.leader(v);
            let black_cnt = *black.get(&leader).unwrap();
            if black_cnt > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}