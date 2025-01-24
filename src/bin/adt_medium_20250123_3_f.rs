use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [usize; n],
    }
    let mut ans = usize::MAX;
    for i in 0..(2_usize.pow(n as u32)) {
        let mut a = 0;
        let mut b = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                a += k[j];
            } else {
                b += k[j];
            }
        }
        if ans > a.max(b) {
            ans = a.max(b);
        }
    }
    println!("{}", ans);
}