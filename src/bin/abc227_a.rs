use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    }
    let mut ans = a-1;
    let mut cnt = 0;
    for _ in 0..k {
        ans += 1;
        cnt += 1;
        if ans > n { ans = 1; }
    }
    println!("{}", ans);
}