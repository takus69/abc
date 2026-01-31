use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }
    let mut ans = 0;
    let mut cnt = n;
    n += 1;
    while cnt < k {
        ans += 1;
        cnt += n;
        n += 1;
    }
    println!("{}", ans);
}