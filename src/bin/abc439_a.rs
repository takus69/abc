use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", 2usize.pow(n as u32)-2*n);
}