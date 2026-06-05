 use proconio::input;

 fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for &ai in &a {
        if ai < p {
            ans += 1;
        }
    }
    println!("{}", ans);
 }