use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    println!("{}", x*2_usize.pow(y as u32));
}