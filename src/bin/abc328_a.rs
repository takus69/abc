use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }
    let mut ans = 0;
    for si in s.iter() {
        if *si <= x {
            ans += si;
        }
    }
    println!("{}", ans);
}