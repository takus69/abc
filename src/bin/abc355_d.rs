use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut events = Vec::new();
    for (l, r) in lr {
        events.push((l, 0));
        events.push((r, 1));  // 開始イベントを先に処理するため、開始が0、終了が1
    }
    events.sort();

    let mut cnt = 0;
    let mut ans = 0;
    for (_, v) in events {
        if v == 0 {
            ans += cnt;
        } else {
            cnt += 1;
        }
    }
    println!("{}", n*(n-1)/2-ans);
}
    /*
    let mut map: HashMap<usize, (i64, i64)> = HashMap::new();
    for (l, r) in lr {
        // 開始: l
        (map.entry(l).or_insert((0, 0))).0 += 1;
        // 終了: r+1
        (map.entry(r+1).or_insert((0, 0))).1 += -1
    }
    
    let mut kv: Vec<_> = map.into_iter().collect();
    kv.sort();

    let mut ans = 0;
    let mut pre = 0;
    let mut not = 0;
    let mut cnt = 0;
    for (_, v) in kv.iter() {
        let next = v.1 + v.0 + pre;
        if v.0 > 0 {
            ans += (pre+v.1) * v.0;
            ans += (v.0 * (v.0-1)) / 2;
        }
        pre = next;
    }
    println!("{}", ans);
}*/