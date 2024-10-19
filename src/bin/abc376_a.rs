use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        t: [usize; n],
    }
    let mut ans = 1;
    let mut pre = t[0];
    for i in 1..n {
        let ti = t[i];
        if ti - pre >= c {
            ans += 1;
            pre = ti;
        }
    }
    println!("{}", ans);
}