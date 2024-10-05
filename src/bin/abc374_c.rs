use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [usize; n],
    }
    let mut ans = usize::MAX;
    for i in 0..1<<n {
        let mut a = 0;
        let mut b = 0;
        for j in 0..n {
            if i>>j & 1 == 1 {
                a += k[j];
            } else {
                b += k[j];
            }
        }
        ans = ans.min(a.max(b));
    }
    println!("{}", ans);
}
