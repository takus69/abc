use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    if s==t {
        println!("Yes");
        return;
    }
    let n = s.len();
    for i in 0..(n-1) {
        let mut s2 = s.clone();
        s2.swap(i, i+1);
        if s2 == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}