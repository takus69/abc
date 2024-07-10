use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans: i64 = 0;
    let mut now: i64 = 0;
    for ai in a.iter() {
        now += ai;
        ans = ans.min(now);
    }
    println!("{}", now-ans);
}