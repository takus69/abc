use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut seet = k;
    for ai in a.iter() {
        if seet >= *ai {
            seet -= ai;
        } else {
            ans += 1;
            seet = k - ai;
        }
    }
    ans += 1;
    println!("{}", ans);
}