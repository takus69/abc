use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    // let i = a.binary_search(&x).unwrap();
    let mut ok = 0;
    let mut ng = n;
    while ok+1 < ng {
        let m = (ok+ng) / 2;
        if a[m] <= x {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok+1);
}