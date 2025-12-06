use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, f64); n],
    }
    let mut ans: Vec<f64> = vec![0.0; m];
    let mut cnt: Vec<usize> = vec![0; m];
    for &(a, b) in ab.iter() {
        ans[a-1] += b;
        cnt[a-1] += 1;
    }
    for i in 0..m {
        ans[i] /= cnt[i] as f64;
    }
    for a in ans.iter() {
        println!("{}", a);
    }
}