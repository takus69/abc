use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    }
    let mut i = 0;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 1 {
            i = (i+x)%n;
        } else {
            println!("{}", s[(x+n-i-1)%n]);
        }

    }
}