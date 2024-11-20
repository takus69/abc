use proconio::input;

fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        q: usize,
    }
    for _ in 0..q {
        input! {
            t: usize,
            d: usize,
        }
        let (q, r) = qr[t-1];
        if d % q == r {
            println!("{}", d);
        } else {
            let ans = if d <= r { r } else { ((d-r-1)/q+1)*q+r };
            println!("{}", ans);
        }
    }
}