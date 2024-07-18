use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    // 各数字の個数をカウント
    let s = s.iter().map(|x| x.to_string().parse().unwrap()).collect::<Vec<usize>>();
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for si in s.iter() {
        let e = cnt.entry(*si).or_insert(0);
        *e += 1;
    }

    // 候補となる平方数をあげ、平方数の各数字の個数が一致する場合は、組合せの数を足す
    let mut ans = 0;
    for i in 0..(10_usize.pow((n/2 + n%2) as u32)) {
        // 候補の平方数
        let c = (i*i).to_string().chars().collect::<Vec<char>>();
        let c = c.iter().map(|x| x.to_string().parse().unwrap()).collect::<Vec<usize>>();

        // 候補の平方数の各数字の個数をカウント
        let mut cnt2: HashMap<usize, usize> = HashMap::new();
        // 桁数が足りない場合は0を追加
        if c.len() < n { cnt2.insert(0, n-c.len()); }
        if c.len() > n { break; }
        for ci in c.iter() {
            let e = cnt2.entry(*ci).or_insert(0);
            *e += 1;
        }

        // 比較
        let mut flg = true;
        for (k, v) in cnt.iter() {
            if !cnt2.contains_key(k) || cnt2[k] != *v {
                flg = false;
            }
        }

        // 組合せを計上
        if flg {
            // println!("cnt: {:?}, cnt2: {:?}, n: {}, c.len: {}", cnt, cnt2, n, c.len());
            let mut ans2 = 1;
            for j in 0..10 {
                if cnt2.contains_key(&i) {
                    ans2 *= cnt2[&i];
                }
            }
            ans += ans2;
        }
    }
    println!("{}", ans);
}