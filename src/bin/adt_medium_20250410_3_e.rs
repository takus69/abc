use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    if n == 1 { println!("1"); return; }
    let mut ans = n;
    let mut cnt = 2;
    let mut r = 1;
    let mut d = a[1] - a[0];
    while r < n-1 {
        let d2 = a[r+1] - a[r];
        if d == d2 {
            cnt += 1;
        } else {
            ans += cnt*(cnt-1)/2;
            cnt = 2;
        }
        d = d2;
        r += 1;
    }
    ans += cnt*(cnt-1)/2;
    println!("{}", ans);
}