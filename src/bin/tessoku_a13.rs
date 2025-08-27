use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for (i, &ai) in a.iter().enumerate() {
        let ai2 = ai + k;
        // ai2 より大きい最初のインデックスを見つける
        let mut ng = n;
        let mut ok = i;
        while ok + 1 < ng {
            let m = (ok + ng) / 2;
            if a[m] <= ai2 {
                ok = m;
            } else {
                ng = m;
            }
        }
        ans += ok - i;
    }
    println!("{}", ans);
}