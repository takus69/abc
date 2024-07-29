use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut ab: [(usize, usize); n],
    }
    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();  // key: (選ぶ個数, 甘さの合計), value: しょっぱさの合計
    dp.insert((0, 0), 0);
    for (a, b) in ab {
        let mut dp2 = dp.clone();
        for i in 0..=n {
            for sum_a in 0..=x {
                if !dp.contains_key(&(i, sum_a)) { continue; }
                let sum_b = dp.get(&(i, sum_a)).unwrap();
                if sum_a + a > x || sum_b + b > y { continue; }
                if dp.contains_key(&(i+1, sum_a+a)) {
                    let sum_b2 = dp.get(&(i+1, sum_a+a)).unwrap();
                    if *sum_b2 > sum_b+b  {
                        dp2.insert((i+1, sum_a+a), sum_b+b);
                    }
                } else {
                    dp2.insert((i+1, sum_a+a), sum_b+b);
                }
            }
        }
        dp = dp2;
    }
    let mut ans = 0;
    // println!("{:?}", &dp);
    for (k, v) in dp {
        if k.1 <= x && v <= y {
            ans = ans.max(k.0);
        }
    }
    println!("{}", n.min(ans+1));
}