use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut j = 1;
    for i in 0..n {
        while j < n && a[i] > a[j]/2 {
            j += 1;
        }
        ans += n - j;
    }
    println!("{}", ans);
}