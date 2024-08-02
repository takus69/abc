use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: [Chars; n],
    }
    let mut pre = a[1][0];
    for j in 0..n {
        std::mem::swap(&mut a[0][j], &mut pre);
    }
    for i in 1..n {
        std::mem::swap(&mut a[i][n-1], &mut pre);
    }
    for j in (0..(n-1)).rev() {
        std::mem::swap(&mut a[n-1][j], &mut pre);
    }
    for i in (0..(n-1)).rev() {
        std::mem::swap(&mut a[i][0], &mut pre);
    }
    for ai in a {
        println!("{}", ai.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}