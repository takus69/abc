use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        mut x: Chars,
    }
    x.reverse();
    let mut comsum: Vec<usize> = vec![0];
    let mut sum = 0;
    for &xi in &x {
        let xi = xi.to_digit(10).unwrap() as usize;
        sum += xi;
        comsum.push(sum);
    }
    let sum_a = *comsum.last().unwrap();
    let mut bi = 0;
    let mut ans: Vec<usize> = Vec::new();
    for (i, &xi) in x.iter().enumerate() {
        let xi = xi.to_digit(10).unwrap() as usize;
        let ai = xi + bi + sum_a - comsum[i+1];
        ans.push(ai%10);
        bi = ai / 10;
    }

    if bi > 0 {
        ans.push(bi);
    }
    ans.reverse();
    println!("{}", ans.iter().join(""));
}