use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans: Vec<usize> = vec![1; k];
    let mut sum = k;
    ans.push(sum);
    let r#mod = 1_000_000_000;
    for i in (k+1)..=n {
        sum += ans[i-1];
        sum -= ans[i-k-1];
        sum += r#mod;
        sum %= r#mod;
        ans.push(sum);
    }
    println!("{}", ans[n]);
}