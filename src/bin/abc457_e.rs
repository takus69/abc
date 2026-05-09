use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
        q: usize,
    }

    // 各端に対する逆側の端のリスト(事前ソートし二分探索に使う)
    let mut l_list: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    let mut r_list: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for &(l, r) in &lr {
        l_list[l].push(r);
        r_list[r].push(l);
    }
    for i in 0..=n {
        l_list[i].sort();
        r_list[i].sort();
    }

    // 各端以上の最小の端
    let mut l_min: Vec<usize> = vec![usize::MAX; n+2];
    let mut lr = lr;
    lr.sort_by(|a, b| b.1.cmp(&a.1));
    for &(l, r) in lr.iter() {
        l_min[l] = r;
    }
    for i in (1..n).rev() {
        l_min[i] = l_min[i].min(l_min[i+1]);
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }
        // lと一致するlrを探す.r以下を二分探索
        if l_list[s].is_empty() {
            println!("No");continue;
        }
        if l_list[s][0] > t {
            println!("No");continue;
        }
        let mut flg = false;
        let mut ok = 0;
        let mut ng = l_list[s].len();
        while ok+1 < ng {
            let m = (ok + ng)/2;
            if l_list[s][m] <= t {
                ok = m;
            } else {
                ng = m;
            }
        }
        let r1 = l_list[s][ok];
        if ok > 0 { flg = true; }
        // println!("s: {}, r1: {}", s, r1);

        // rと一致するlrを探す.l以上を二分探索
        if r_list[t].is_empty() {
            println!("No");continue;
        }
        if r_list[t].last().unwrap() < &s {
            println!("No");continue;
        }
        let mut ok = r_list[t].len()-1;
        let mut ng = 0;
        while ok > ng+1 {
            let m = (ok + ng)/2;
            if r_list[t][m] >= s {
                ok = m;
            } else {
                ng = m;
            }
        }
        let l1 = if r_list[t][0] >= s { r_list[t][0] } else { r_list[t][ok] };
        // println!("l1: {}, t: {}", l1, t);

        let ok = if l1 <= r1+1 {
            let mut ok = false;
            ok |= l1 != s || r1 != t;  // 違う2つの布
            ok |= l1 == s && r1 == t && flg;  // 全部覆う布2枚
            ok |= !flg && (l_min[s+1] <= t || l_min[s] < t);  // 全部覆う布1枚で、狭い範囲の布1枚
            ok } else { false };
        if ok {
            println!("Yes");
        } else {
            println!("No");
        }

    }
}