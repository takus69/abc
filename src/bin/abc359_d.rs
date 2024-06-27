use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    let r#mod = 998244353;
    let mut patterns = vec!["".to_string()];
    let mut tmp;
    for _ in 0..k {
        tmp = vec![];
        for p in patterns {
            for a in ["A", "B"] {
                tmp.push(format!("{}{}", p, a));
            }
        }
        patterns = tmp;
    }
    let mut tmp = Vec::new();
    for p in patterns {
        let mut flg = true;
        for i in 0..(k/2) {
            if p[i..(i+1)] != p[(p.len()-i-1)..(p.len()-i)] {
                flg = false;
                break;
            }
        }
        if !flg { tmp.push(p); }
    }
    patterns = tmp;
    // println!("{:?}", patterns);

    let mut dp: HashMap<usize, HashMap<String, usize>> = HashMap::new();

    for i in 0..=(n-k) {
        dp.insert(i, HashMap::new());
        for p in patterns.iter() {
            dp.get_mut(&i).unwrap().insert(p.to_string(), 0);
        }
        let si = &s[i..(i+k)];
        for p in patterns.iter() {
            let mut flg = true;
            for j in 0..k {
                if &si[j..(j+1)] == &p[j..(j+1)] || &si[j..(j+1)] == "?" {
                    continue;
                } else {
                    flg = false;
                    break;
                }
            }
            if flg {
                if i == 0 {
                    dp.get_mut(&i).unwrap().insert(p.to_string(), 1);
                } else {
                    let tmp_a = match dp[&(i-1)].get(&format!("A{}", &p[0..(k-1)])) {
                        Some(value) => *value,
                        None => 0,
                    };
                    let tmp_b = match dp[&(i-1)].get(&format!("B{}", &p[0..(k-1)])) {
                        Some(value) => *value,
                        None => 0,
                    };
                    let cnt = dp.get_mut(&i).unwrap().entry(p.to_string()).or_insert(0);
                    *cnt += tmp_a + tmp_b;
                    *cnt %= r#mod;
                }
            }
        }
    }


    let mut ans = 0;
    for p in patterns {
        ans += dp[&(n-k)][&p];
        ans %= r#mod;
    }
    // println!("{:?}", dp);
    println!("{}", ans);
}