use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", (1..=n).rev().collect::<Vec<usize>>().into_iter().join(","));
}