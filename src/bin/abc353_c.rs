use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let r#mod = 100000000;
    for ai in a.iter_mut() {
        *ai %= r#mod;
    }
    // (n-1)*sum(a)
    let mut ans: usize = a.iter().sum();
    ans *= n-1;

    // 足して10^8を超える個数を数える
    a.sort();
    let mut l = 0;
    let mut r = n-1;
    let mut cnt = 0;
    while l < r {
        if a[l] + a[r] >= r#mod {
            cnt += r - l;
            r -= 1;
        } else {
            l += 1;
        }
    }
    ans -= r#mod * cnt;

    println!("{}", ans);
}