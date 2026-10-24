use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut pre_a = a[0];
    let mut ans = if pre_a == k { 1 } else { 0 };
    for &ai in a.iter().skip(1) {
        if (pre_a > k && ai <= k) || (pre_a < k && ai >= k) {
            ans += 1;
        }
        pre_a = ai;
    }

    println!("{}", ans);
}