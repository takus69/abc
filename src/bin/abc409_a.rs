use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        a: Chars,
    }
    for i in 0..n {
        if t[i] == 'o' && a[i] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}