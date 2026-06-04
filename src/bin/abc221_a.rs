use proconio::input;

fn main() {
    input! {
        a: usize,
        mut b: usize,
    }
    println!("{}", 32_usize.pow((a-b) as u32));
}