use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ng = 0;
    let mut ok = 1_000_000_001;
    while ng + 1 < ok {
        let m = (ok + ng) / 2;
        let mut tmp = 0;
        for &ai in a.iter() {
            tmp += m / ai;
        }
        if tmp < k {
            ng = m;
        } else {
            ok = m;
        }
    }
    println!("{}", ok);
}