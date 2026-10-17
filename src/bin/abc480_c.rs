use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        t: isize,
        mut a: [isize; n],
    }
    a.sort();
    let mut ans = if a[0] == 0 { 0 } else { (a[0].min(t)-1) / k + 1};
    let mut ok = a[0] + k;
    for &ai in a.iter() {
        // println!("ans: {}, ok: {}, ai: {}", ans, ok, ai);
        if ai > ok {
            if ai <= t {
                ans += (ai - ok - 1) / k + 1;
            } else {
                ans += (t - ok - 1) / k + 1;
            }
        }
        ok = ai+k;
        if ok >= t { break; }
    }
    // println!("ans: {}, ok: {}", ans, ok);
    if ok < t {
        ans += (t - ok - 1) / k + 1;
    }
    println!("{}", ans);
}