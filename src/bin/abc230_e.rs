use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut i: u64 = 1;
    let mut ans: u64 = 0;
    while i <= n {
        let d = n / i;
        let cnt = n/d - n/(d+1);
        ans += cnt * d;
        i += cnt;
    }
    println!("{}", ans);
}
