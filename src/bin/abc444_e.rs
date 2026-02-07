use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut set: BTreeSet<usize> = BTreeSet::new();
    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    let mut cnt = 0;
    while l < n && r < n {
        let ai = a[r];
        let s = if ai >= d { ai - d + 1 } else { 0 };
        let t = ai + d - 1;
        match set.range(s..=t).next() {
            Some(_) => {
                ans += cnt-1;
                set.remove(&a[l]);
                l += 1;
                cnt -= 1;
                // println!("Some: l: {}, r: {}, ans: {}, cnt: {}, set: {:?}", l, r, ans, cnt, set);
            },
            None => {
                set.insert(ai);
                r += 1;
                cnt = r-l;
                // println!("None: l: {}, r: {}, ans: {}, cnt: {}, set: {:?}", l, r, ans, cnt, set);
            }
        }
    }
    // println!("ans: {}, l: {}, r: {}", ans, l, r);
    ans += cnt*(cnt-1)/2;

    ans += n;
    println!("{}", ans);
}