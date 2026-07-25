use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut all: Vec<Vec<usize>> = Vec::new();
    for perm in (1..=n).permutations(n) {
        all.push(perm);
    }
    all.sort();
    let mut flg = false;
    let mut ans = 0;
    for v in all.iter() {
        if v == &q {
            flg = false;
            break;
        }
        if flg { ans += 1; }
        if v == &p {
            flg = true;
        }
    }
    println!("{}", ans);
}