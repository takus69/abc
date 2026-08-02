use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    /**
    // 愚直
    let mut ans = 0;
    for a in 1..n {
        for b in (a+1)..= n{
            let mut flg = true;
            for &(a2, b2) in &ab {
                if a != a2 && a != b2 && b != a2 && b != b2 {
                    flg = false;
                    break;
                }
            }
            if flg { ans += 1; }
        }
    }
    println!("ans: {}", ans);
    */
    
    let (a, b) = ab[0];
    let mut cand: HashSet<(usize, usize)> = HashSet::new();
    cand.insert((a, 0));
    cand.insert((b, 0));
    for &(a, b) in ab.iter().skip(1) {
        let mut next_cand = HashSet::new();
        for &c in &cand {
            if c.1 == 0 {
                if c.0 != a && c.0 != b {
                    let (a2, b2) = if c.0 < a { (c.0, a) } else { (a, c.0) };
                    next_cand.insert((a2, b2));
                    let (a2, b2) = if c.0 < b { (c.0, b) } else { (b, c.0) };
                    next_cand.insert((a2, b2));
                } else {
                    next_cand.insert((c.0, 0));
                }
            } else {
                if c.0 == a || c.1 == a || c.0 == b || c.1 == b {
                    next_cand.insert(c);
                }
            }
        }
        cand = next_cand;
        // println!("cand: {:?}", cand);
    }
    let mut cnt0 = 0;
    let mut ans: HashSet<(usize, usize)> = HashSet::new();
    for &c in &cand {
        if c.1 == 0 {
            cnt0 += 1;
            continue;
        }
        // let c = if c.0 < c.1 { c } else { (c.1, c.0) };
        ans.insert(c);
    }
    // println!("cand: {:?}, cnt0: {}", cand, cnt0);

    if cnt0 == 2 {
        println!("{}", 2*(n-1)-1);
    } else if cnt0 == 1 {
        println!("{}", n-1);
    } else {
        println!("{}", ans.len());
    }
}