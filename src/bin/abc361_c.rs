use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = usize::MAX;
    for i in 0..=k {
        let tmp = a[n-1-i] - a[k-i];
        if tmp < ans { ans = tmp; }
    }
    println!("{}", ans);
}