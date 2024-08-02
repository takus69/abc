use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let sum_a: usize = a.iter().sum();
    let ave = sum_a / n;
    let exc = sum_a % n;
    a.sort();
    let mut ans = 0;
    let mut cnt = 0;
    for ai in a.iter() {
        if ave >= *ai {
            cnt += 1;
            ans += ave - ai;
        }
    }
    // println!("n: {}, cnt: {}, ave: {}, exc: {}", n, cnt, ave, exc);
    if n-cnt < exc {
        ans += exc - (n-cnt);
    }
    println!("{}", ans);
}