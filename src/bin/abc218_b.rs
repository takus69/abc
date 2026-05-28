use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        p: [u8; 26],
    }
    fn i2c(i: u8) -> char {
        (b'a'+ i-1) as char
    }
    println!("{}", p.iter().map(|&x| i2c(x)).join(""));
}