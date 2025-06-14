use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }
    let mut ans = 0;
    for &ai in a.iter() {
        if k <= ai {
            ans += 1;
        }
    }
    println!("{}", ans);
}