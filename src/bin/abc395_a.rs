use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = "Yes";
    let mut pre = a[0];
    for i in 1..n {
        if pre >= a[i] {
            ans = "No";
        }
        pre = a[i];
    }
    println!("{}", ans);
}