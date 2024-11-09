use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize; m],
        a: [usize; m],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..m {
        map.insert(x[i]-1, a[i]);
    }
    x.sort();

    let mut ans = 0;
    let mut cnt = 0;
    let mut pre = n;
    for j in (0..m).rev() {
        let i = x[j]-1;
        cnt += pre - i - 1;
        pre = i;
        let bi = if map.contains_key(&i) { *map.get(&i).unwrap() } else { 0 };
        // println!("ans: {}, cnt: {}, bi: {}", ans, cnt, bi);
        if bi == 0 {
            cnt += 1;
        } else {
            if cnt == 0 && bi == 1 {
                continue;
            }
            if cnt >= bi {
                ans += bi * (cnt*2 + 1 - bi) / 2;
                cnt = cnt + 1 - bi;
            } else if cnt+1 == bi {
                ans += cnt * (cnt+1) / 2;
                cnt = 0;
            } else {
                println!("-1");
                std::process::exit(0);
            }
        }
    }
    
    // println!("ans: {}, cnt: {}", ans, cnt);
    if cnt > 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}