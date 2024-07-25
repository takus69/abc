use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }
    if n < m {
        println!("0");
    } else {
        println!("{}", (n-m)/p + 1);
    }
}