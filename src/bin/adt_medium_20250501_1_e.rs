use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }
    let mut sum = 0;
    let mut ans = 0;
    for i in 0..m {
        sum += a[i];
        ans += (i+1) as isize *a[i];
    }
    let mut sum_a: Vec<isize> = vec![sum];
    for i in 0..(n-m) {
        sum -= a[i];
        sum += a[i+m];
        sum_a.push(sum);
    }
    let mut max_ans= ans;
    for i in m..n {
        ans -= sum_a[i-m];
        ans += m as isize *a[i];
        max_ans = max_ans.max(ans);
    }
    println!("{}", max_ans);
}