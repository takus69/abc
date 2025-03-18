use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }
    let mut ans = t[0];
    ans += a;
    println!("{}", ans);
    for i in 1..n {
        let ti = t[i];
        if ti < ans {
            ans += a;
        } else {
            ans = ti + a;
        }
        println!("{}", ans);
    }
}