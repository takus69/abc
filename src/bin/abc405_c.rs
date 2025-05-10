use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = 0;
    for &ai in a.iter() {
        ans += ai * (sum_a-ai);
    }
    ans /= 2;
    println!("{}", ans);
}