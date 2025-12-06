use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = "No";
    for &ai in a.iter() {
        if sum_a-ai == m {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}