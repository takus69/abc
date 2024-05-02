use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    let mut cnt = 0;
    while r < n {
        // println!("l: {}, r: {}, al: {}, ar: {}", l, r, a[l], a[r]);
        if a[l] < a[r] {
            cnt += 1;
            l = r;
            r += 1;
        } else {
            ans += cnt * (cnt + 1) / 2;
            cnt = 0;
            l = r;
        }
        if l == r {
            r += 1;
        }
        // println!("cnt: {}", cnt);
    }
    ans += cnt * (cnt + 1) / 2;
    ans += n;
    println!("{}", ans);
}