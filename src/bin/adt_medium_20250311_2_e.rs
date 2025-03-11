use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = 1;
    let mut cnt = 1;
    let mut l = 0;
    let mut r = 1;
    while r < n {
        let al = a[l];
        let ar = a[r];
        if ar - al >= m {
            l += 1;
            cnt -= 1;
        } else {
            cnt += 1;
            ans = ans.max(cnt);
            r += 1;
        }
    }
    println!("{}", ans);
}