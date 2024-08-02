use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main()  {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut set: HashSet<String> = HashSet::new();
    let mut cnt = 0;
    for si in s.iter() {
        let rev_si = si.chars().rev().collect::<String>();
        // 全文字同じかを確認
        if *si == rev_si && !set.contains(si) { cnt += 1 };

        // そのままの文字列と逆順の文字列を入れる
        set.insert(si.clone());
        set.insert(rev_si.clone());
    }
    println!("{}", (set.len() + cnt)/2);
}