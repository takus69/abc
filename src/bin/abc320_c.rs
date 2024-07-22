use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        m: usize,
        mut s: [Chars; 3],
    }
    let s = s.iter().map(|x| x.iter().map(|y| y.to_string().parse().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();
    let mut ans = usize::MAX;
    for p in (0..3).permutations(3) {  // リールの押す順番を全て試す
        for (t1, c1) in s[p[0]].iter().enumerate() {  // 1個目のリールで揃える数を決める
            for t2 in (t1+1)..=(t1+m) {
                if s[p[1]][t2%m] == *c1 {
                    for t3 in (t2+1)..=(t2+m) {
                        if s[p[2]][t3%m] == *c1 {
                            ans = ans.min(t3);
                            // println!("ans: {}, c1: {}, p: {:?}, t1: {}, t2: {}, t3: {}", ans, c1, p, t1, t2, t3);
                        }
                    }
                }
            }
        }
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}