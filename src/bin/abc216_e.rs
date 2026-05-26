use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut a: [usize; n],
    }
    // 二分探索解
    // m以上の個数を二分探索、k個以下となる境界を探す
    let mut ng = 0;
    let mut ok = 2_000_000_001;
    while ng+1 < ok {
        let m = (ng + ok) / 2;
        let mut cnt = 0;
        for &ai in &a {
            if ai >= m {
                cnt += ai-m+1;
            }
        }
        if cnt > k {
            ng = m;
        } else {
            ok = m;
        }
    }
    let mut ans = 0;
    for &ai in &a {
        if ai >= ok {
            ans += (ai+ok) * (ai-ok+1) / 2;
            k -= ai-ok+1;
        }
    }
    ans += k*(ok-1);
    println!("{}", ans);
    
    /* 自分のコード
    a.push(0);
    a.sort();a.reverse();

    let mut ans = 0;
    let mut same_cnt = 0;
    let mut same_ai = a[0];
    for &ai in &a {
        // println!("ans: {}, same_cnt: {}, same_ai: {}, ai: {}", ans, same_cnt, same_ai, ai);
        if same_ai != ai {
            let diff = same_ai - ai;
            if same_cnt * diff >= k {
                let diff2 = k / same_cnt;
                let get_sum_ai = same_cnt * diff2 * (same_ai + (same_ai-diff2) + 1) / 2;
                ans += get_sum_ai;
                ans += (k%same_cnt)*(same_ai-diff2);
                break;
            } else {
                let get_sum_ai = same_cnt * diff * (same_ai + ai + 1) / 2;
                ans += get_sum_ai;
                k -= same_cnt * diff;
            }
        }
        same_cnt += 1;
        same_ai = ai;
    }

    println!("{}", ans);
    */
}