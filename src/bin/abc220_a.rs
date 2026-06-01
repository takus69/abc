use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    for i in 1..=1000 {
        let ans = c*i;
        if a <= ans && ans <= b {
            println!("{}", ans);
            return;
        }
    }
    println!("-1");
}