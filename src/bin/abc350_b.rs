use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut ans = vec![1; n];
    for ti in t {
        ans[ti-1] = (ans[ti-1] + 1) % 2;
    }
    println!("{}", ans.iter().sum::<i32>());
}